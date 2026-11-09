use std::fs::File;
use std::net::SocketAddr;
use std::time::Instant;

use kaeru_core::{
    KaeruConfig, Store,
    api::{ApiOptions, execute},
    config::{StorageConfig, storage::TiKvConfig},
    new_node_id,
};
use serde_json::json;
use tempfile::tempdir;

use super::Result;
use super::cli::Seed;
use super::models::{Card, Fixture};

pub fn seed(args: Seed) -> Result<()> {
    if args.pd_endpoints.is_empty()
        || args.pd_endpoints.iter().any(|v| {
            v.parse::<SocketAddr>()
                .map_or(true, |a| !a.ip().is_loopback())
        })
    {
        return Err("seed requires explicit loopback PD endpoints for an isolated lab".into());
    }
    let output = File::create_new(&args.output)?;
    let namespace = format!("known-read-{}", new_node_id());
    let temp = tempdir()?;
    let config = KaeruConfig {
        vault_path: temp.path().join("vault"),
        storage: StorageConfig::Tikv {
            connection: TiKvConfig {
                end_points: args.pd_endpoints,
                namespace: Some(namespace.clone()),
                ..TiKvConfig::default()
            },
        },
        read_trace_enabled: false,
        ..KaeruConfig::defaults()
    };
    let started = Instant::now();
    let store = Store::open_with_config(config)?;
    let mut fixture = populate(&store, args.records as usize)?;
    fixture.namespace = namespace;
    fixture.backend = store.config().storage.kind().into();
    drop(store);
    fixture.seed_seconds = started.elapsed().as_secs_f64();
    serde_json::to_writer_pretty(output, &fixture)?;
    println!(
        "fixture records={} versions={} edges={} backend={} seed_seconds={:.3}",
        fixture.records,
        fixture.versions_per_record,
        fixture.edges,
        fixture.backend,
        fixture.seed_seconds
    );
    Ok(())
}

pub fn populate(store: &Store, records: usize) -> Result<Fixture> {
    let ids = (0..records).map(|_| new_node_id()).collect::<Vec<_>>();
    let mut cards = Vec::new();
    let mut rows = Vec::new();
    let mut membership = Vec::new();
    for (index, id) in ids.iter().enumerate() {
        let bytes: usize = match index {
            0 => 1024,
            5 => 4096,
            10 => 16384,
            _ => 128,
        };
        let prefix = format!("Synthetic observation {index}. ");
        let mut body = prefix.repeat(bytes.div_ceil(prefix.len()));
        body.truncate(bytes);
        let scope = format!("known-read-{}", index % 5);
        let name = format!("Synthetic observation {index}");
        rows.push(json!([id, name, body, scope]));
        membership.push(json!([scope, id]));
        if matches!(index, 0 | 5 | 10) {
            cards.push(Card {
                id: id.clone(),
                name,
                scope,
                body,
                expected_conflicts: 1,
                expected_reasoning: true,
            });
        }
    }
    let script = format!(
        "input[id,name,body,scope] <- {}
?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] := input[id,name,body,scope], v in int_range(0,2),validity=[1000000000+v*1000000,true],type='episode',tier='operational',tags=[],initiatives=null,properties=null,visibility='local',layer='warm' :put node {{id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}}",
        serde_json::to_string(&rows)?
    );
    store.run(&script)?;
    store.run(&format!(
        "?[initiative,node_id] <- {} :put node_initiative {{initiative,node_id}}",
        serde_json::to_string(&membership)?
    ))?;
    let mut edges = Vec::new();
    for group in 0..5 {
        let group_ids = ids
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 5 == group)
            .map(|(_, id)| id)
            .collect::<Vec<_>>();
        for (n, id) in group_ids.iter().enumerate() {
            for step in [1, 2] {
                edges.push(json!([
                    id,
                    group_ids[(n + step) % group_ids.len()],
                    "refers_to"
                ]));
            }
        }
    }
    for card in &cards {
        edges.push(json!([ids[15], card.id, "falsifies"]));
    }
    store.run(&format!("input[src,dst,edge_type] <- {}
?[src,dst,edge_type,validity,weight,properties] := input[src,dst,edge_type],validity=[1000000000,true],weight=0.7,properties=null :put edge {{src,dst,edge_type,validity=>weight,properties}}",serde_json::to_string(&edges)?))?;
    let members = cards.iter().map(|c| json!({"id":c.id})).collect::<Vec<_>>();
    let trail = execute(
        store,
        "trail",
        json!({"initiative":"known-read-0","route":{"members":members},"name":"Synthetic benchmark reasoning","summary":"Synthetic observations form a retained reasoning trail for the known-card fixture."}),
        &ApiOptions::local(),
    );
    if trail.is_error() {
        return Err("fixture trail creation failed".into());
    }
    for card in &cards {
        let read = execute(
            store,
            "read",
            json!({"initiative":card.scope,"node":{"id":card.id},"budget_chars":64000}),
            &ApiOptions::local(),
        );
        let value = serde_json::to_value(read)?;
        if value.pointer("/data/node/body").and_then(|v| v.as_str()) != Some(&card.body)
            || value
                .pointer("/data/conflicts")
                .and_then(|v| v.as_array())
                .map(|v| v.len())
                != Some(card.expected_conflicts)
            || value
                .pointer("/data/reasoning")
                .and_then(|v| v.as_array())
                .is_none_or(|v| v.is_empty())
        {
            return Err("seeded card does not satisfy its independent fixture contract".into());
        }
    }
    let denied = Card {
        id: ids[1].clone(),
        name: "Synthetic observation 1".into(),
        scope: "known-read-1".into(),
        body: String::new(),
        expected_conflicts: 0,
        expected_reasoning: false,
    };
    Ok(Fixture {
        schema_version: 1,
        backend: store.config().storage.kind().into(),
        namespace: String::new(),
        allowed_scope: "known-read-0".into(),
        records,
        versions_per_record: 2,
        edges: edges.len(),
        cards,
        denied,
        seed_seconds: 0.0,
    })
}

#[cfg(test)]
mod tests {
    use super::populate;
    use kaeru_core::{KaeruConfig, Store};
    use tempfile::tempdir;
    #[test]
    fn synthetic_fixture_has_full_bodies_conflict_and_saved_reasoning() {
        let dir = tempdir().unwrap();
        let store = Store::open_in_memory_with(KaeruConfig {
            vault_path: dir.path().join("vault"),
            ..KaeruConfig::defaults()
        })
        .unwrap();
        let fixture = populate(&store, 30).unwrap();
        assert_eq!(
            fixture
                .cards
                .iter()
                .map(|c| c.body.len())
                .collect::<Vec<_>>(),
            vec![1024, 4096, 16384]
        );
    }
}
