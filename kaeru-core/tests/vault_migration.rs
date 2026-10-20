use std::fs;

use kaeru_core::{
    Layer, NodeType, Store, Tier, Visibility, api, history, improve, jot, new_node_id, upsert_node,
};
use serde_json::json;

#[test]
fn opening_a_pre_api_vault_preserves_versions_and_freezes_legacy_terminal_meaning() {
    let dir = std::env::temp_dir().join(format!("kaeru-migration-{}", new_node_id()));
    let (id, before, global, retired, global_history);
    {
        let store = Store::open(&dir).unwrap();
        store.use_initiative("existing-project");
        id = jot(&store, "Original observation.").unwrap();
        improve(&store, &id, "renamed-observation", "Corrected observation.").unwrap();
        before = history(&store, &id).unwrap();
        global = store.scoped(None, |s| jot(s, "An existing unscoped record.").unwrap());
        retired = store.scoped(None, |s| {
            let id = jot(s, "An unscoped retired record.").unwrap();
            kaeru_core::forget(s, &id).unwrap();
            id
        });
        global_history = history(&store, &global).unwrap();
        let board = new_node_id();
        upsert_node(&store,&board,NodeType::Board,Tier::Operational,"board-existing-project",None,&[],Some("existing-project"),Visibility::Local,Layer::Warm,Some(&json!({"statuses":[{"key":"open","label":"Open"},{"key":"closed","label":"Closed"}]}))).unwrap();
        // Recreate the actual schema boundary: v0.7.5 had these graph relations
        // and no API metadata relations or 0010 migration-journal entry.
        for relation in [
            "api_operation",
            "api_slot_revision",
            "api_trail_summary",
            "api_hypothesis_decision",
        ] {
            store.run(&format!("::remove {relation}")).unwrap();
        }
        store
            .run("?[name] <- [['0010_canonical_api_metadata'],['0011_scope_legacy_unscoped']] :rm migration_journal {name}")
            .unwrap();
    }
    {
        let store = Store::open(&dir).unwrap();
        assert_eq!(
            history(&store, &id)
                .unwrap()
                .iter()
                .map(|v| (&v.name, &v.body, v.seconds, v.asserted))
                .collect::<Vec<_>>(),
            before
                .iter()
                .map(|v| (&v.name, &v.body, v.seconds, v.asserted))
                .collect::<Vec<_>>()
        );
        assert!(
            kaeru_core::list_initiatives(&store)
                .unwrap()
                .contains(&"legacy-unscoped".into())
        );
        assert_eq!(
            history(&store, &global).unwrap().len(),
            global_history.len()
        );
        let recovered = api::execute(
            &store,
            "read",
            json!({"initiative":"legacy-unscoped","node":global}),
            &api::ApiOptions::local(),
        );
        assert!(!recovered.is_error(), "{recovered:?}");
        let old = api::execute(
            &store,
            "history",
            json!({"initiative":"legacy-unscoped","node":retired}),
            &api::ApiOptions::local(),
        );
        assert!(!old.is_error(), "{old:?}");
        let past = api::execute(
            &store,
            "read",
            json!({"initiative":"existing-project","node":id,"recorded_at":chrono::DateTime::<chrono::Utc>::from_timestamp(before[0].seconds as i64,0).unwrap().to_rfc3339()}),
            &api::ApiOptions::local(),
        );
        assert!(!past.is_error(), "{past:?}");
        match past.data {
            api::ResponseData::Record(data) => {
                assert_eq!(data.node.body.as_deref(), Some("Original observation."))
            }
            _ => panic!(),
        }
        let added = api::execute(
            &store,
            "board_config",
            json!({"initiative":"existing-project","action":"add","key":"review"}),
            &api::ApiOptions::local(),
        );
        assert!(!added.is_error(), "{added:?}");
        let statuses = kaeru_core::effective_statuses(&store, "existing-project").unwrap();
        assert!(
            statuses
                .iter()
                .find(|s| s.key == "closed")
                .unwrap()
                .terminal
        );
        assert!(
            !statuses
                .iter()
                .find(|s| s.key == "review")
                .unwrap()
                .terminal
        );
        let record = api::execute(
            &store,
            "note",
            json!({"initiative":"existing-project","body":"New operation after migration.","request_id":"after-migration"}),
            &api::ApiOptions::local(),
        );
        assert!(!record.is_error(), "{record:?}");
    }
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn recovered_custom_ids_and_deleted_identity_history_remain_accessible_to_the_owner() {
    let dir = std::env::temp_dir().join(format!("kaeru-legacy-id-{}", new_node_id()));
    {
        let store = Store::open(&dir).unwrap();
        store.run("?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] <- [['custom-legacy-id',[1.0,true],'reference','archival','different-display-name','Original legacy content.',null,null,null,'local','warm']] :put node {id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}").unwrap();
        store
            .run("?[name] <- [['0011_scope_legacy_unscoped']] :rm migration_journal {name}")
            .unwrap();
    }
    {
        let store = Store::open(&dir).unwrap();
        let options = api::ApiOptions::local();
        for node in [json!({"id":"custom-legacy-id"}), json!("custom-legacy-id")] {
            let read = api::execute(
                &store,
                "read",
                json!({"initiative":"legacy-unscoped","node":node}),
                &options,
            );
            assert!(!read.is_error(), "{read:?}");
        }
        let args = json!({"initiative":"legacy-unscoped","node":{"id":"custom-legacy-id"},"enabled":true,"reason":"Keep legacy material visible.","request_id":"legacy-pin"});
        let pinned = api::execute(&store, "pin", args.clone(), &options);
        assert!(!pinned.is_error(), "{pinned:?}");
        let replay = api::execute(&store, "pin", args, &options);
        assert!(replay.replayed, "{replay:?}");
        let delete = api::execute(
            &store,
            "initiative_manage",
            json!({"action":"delete","name":"legacy-unscoped"}),
            &options,
        );
        assert!(!delete.is_error(), "{delete:?}");
        let history = api::execute(
            &store,
            "history",
            json!({"initiative":"legacy-unscoped","node":{"id":"custom-legacy-id"}}),
            &options,
        );
        assert!(!history.is_error(), "{history:?}");
        let restricted = api::ApiOptions {
            allowed_initiatives: Some(vec!["legacy-unscoped".into()]),
            ..options
        };
        let denied = api::execute(
            &store,
            "history",
            json!({"initiative":"legacy-unscoped","node":{"id":"custom-legacy-id"}}),
            &restricted,
        );
        assert!(denied.is_error());
    }
    fs::remove_dir_all(dir).unwrap();
}
