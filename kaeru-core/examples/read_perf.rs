//! Reproducible public-API fixture. Never opens an existing/user vault.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;

use clap::{Parser, ValueEnum};
use kaeru_core::api::{ApiOptions, ResponseData, execute};
use kaeru_core::{KaeruConfig, Store};
use serde_json::{Value, json};
use tempfile::tempdir;

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Backend {
    Mem,
    Rocksdb,
}

#[derive(Parser)]
#[command(about = "Measure public reads against disposable synthetic memory")]
struct Arguments {
    #[arg(long, default_value_t = 2300, value_parser = clap::value_parser!(u32).range(38..=100_000))]
    nodes: u32,
    #[arg(long, value_enum, default_value_t = Backend::Mem)]
    backend: Backend,
    #[arg(long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(1..=100))]
    repeats: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();
    let temporary = tempdir()?;
    let config = KaeruConfig {
        vault_path: temporary.path().join("vault"),
        ..KaeruConfig::default()
    };
    let store = match args.backend {
        Backend::Mem => Store::open_in_memory_with(config)?,
        Backend::Rocksdb => Store::open_with_config(config)?,
    };
    let n = args.nodes;
    // Fixed, past recorded values make baseline/fixed revisions comparable.
    store.run(&format!("?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] := i in int_range(0,{n}),v in int_range(0,6),id=concat('node-',to_string(i)),validity=[1792578012.0+v,true],type='episode',tier='operational',name=concat('Observation ',to_string(i)),body='Synthetic performance observation',tags=[],initiatives=null,properties=null,visibility='local',layer='warm' :put node {{id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}}"))?;
    store.run(&format!("?[initiative,node_id] := i in int_range(0,{n}),initiative=concat('perf-',to_string(i%38)),node_id=concat('node-',to_string(i)) :put node_initiative {{initiative,node_id}}"))?;
    store.run(&format!("?[src,dst,edge_type,validity,weight,properties] := i in int_range(0,{n}),j in [1,2],src=concat('node-',to_string(i)),dst=concat('node-',to_string((i+38*j)%{n})),edge_type='refers_to',validity=[1792578012.0,true],weight=0.5,properties=null :put edge {{src,dst,edge_type,validity=>weight,properties}}"))?;
    assert!(
        !execute(
            &store,
            "context",
            json!({"view":"diagnostics"}),
            &ApiOptions::local()
        )
        .is_error()
    );
    for repeat in 0..args.repeats {
        for (name, input) in [
            (
                "map",
                json!({"initiative":"perf-0","view":"map","budget_chars":64000}),
            ),
            ("work", json!({"initiative":"perf-0","budget_chars":64000})),
            (
                "read",
                json!({"initiative":"perf-0","node":{"id":"node-0"},"include":["relations","reasoning"],"budget_chars":64000}),
            ),
        ] {
            let operation = if name == "read" { "read" } else { "context" };
            let mut input: Value = input;
            let (mut pages, mut returned) = (0, 0);
            let mut digest = DefaultHasher::new();
            let start = Instant::now();
            loop {
                let response = execute(&store, operation, input, &ApiOptions::local());
                assert!(!response.is_error(), "{response:?}");
                pages += 1;
                let records = match &response.data {
                    ResponseData::Context(data) => data
                        .sections
                        .iter()
                        .flat_map(|s| &s.records)
                        .collect::<Vec<_>>(),
                    ResponseData::Record(data) => vec![&data.node],
                    _ => vec![],
                };
                returned += records.len();
                for record in records {
                    (
                        &record.id,
                        &record.revision,
                        &record.body,
                        &record.currentness,
                        &record.initiatives,
                    )
                        .hash(&mut digest);
                }
                if let Some(next) = response.continuation {
                    input = next.next_arguments;
                } else {
                    break;
                }
            }
            println!(
                "{}",
                json!({"backend":format!("{:?}",args.backend).to_lowercase(),"profile":if cfg!(debug_assertions){"debug"}else{"release"},"nodes":n,"node_versions":6*n,"edges":2*n,"scope_nodes":n.div_ceil(38),"case":name,"repeat":repeat,"elapsed_ms":start.elapsed().as_secs_f64()*1000.0,"pages":pages,"returned":returned,"fingerprint":format!("{:016x}",digest.finish())})
            );
        }
    }
    Ok(())
}
