#![cfg(feature = "storage-tikv")]

use std::env;

use kaeru_core::{
    EpisodeKind, Error, KaeruConfig, Significance, Store, at,
    config::storage::{StorageConfig, TiKvConfig, TiKvTlsConfig},
    forget, fuzzy_recall, history, improve, read_node_full, write_episode,
};
use tokio::runtime::Runtime;
use uuid::Uuid;

fn config(optimistic: bool) -> KaeruConfig {
    let mut connection = TiKvConfig {
        end_points: vec![env::var("MNESTIC_TIKV_PD_ENDPOINT").expect("set test PD endpoint")],
        namespace: Some(format!("core-test-{}", Uuid::new_v4())),
        optimistic,
        scan_batch_size: 3,
        ..Default::default()
    };
    if let (Ok(ca), Ok(cert), Ok(key)) = (
        env::var("MNESTIC_TIKV_CA"),
        env::var("MNESTIC_TIKV_CERT"),
        env::var("MNESTIC_TIKV_KEY"),
    ) {
        connection.tls = Some(TiKvTlsConfig {
            ca: ca.into(),
            cert: cert.into(),
            key: key.into(),
        });
    }
    KaeruConfig {
        storage: StorageConfig::Tikv { connection },
        ..KaeruConfig::defaults()
    }
}

#[test]
#[ignore = "requires a disposable TiKV cluster"]
fn async_host_supports_history_rollback_fts_and_reopen() {
    let runtime = Runtime::new().unwrap();
    for optimistic in [true, false] {
        let config = config(optimistic);
        runtime.block_on(async {
            let store = Store::open_with_config(config.clone()).unwrap();
            store.use_initiative("fixture");
            let id = write_episode(
                &store,
                EpisodeKind::Observation,
                Significance::Medium,
                "searchable",
                "alpha first",
            )
            .unwrap();
            let first = history(&store, &id)
                .unwrap()
                .iter()
                .map(|v| v.seconds)
                .max_by(f64::total_cmp)
                .unwrap();
            improve(&store, &id, "searchable", "beta second").unwrap();
            assert_eq!(
                at(&store, &id, first).unwrap().unwrap().body.as_deref(),
                Some("alpha first")
            );
            assert_eq!(
                read_node_full(&store, &id)
                    .unwrap()
                    .unwrap()
                    .body
                    .as_deref(),
                Some("beta second")
            );
            assert!(
                fuzzy_recall(&store, "beta", 10)
                    .unwrap()
                    .iter()
                    .any(|v| v.id == id)
            );
            let failure: kaeru_core::Result<()> = store.atomic(|s| {
                improve(s, &id, "aborted", "gamma never committed")?;
                Err(Error::Invalid("injected rollback".into()))
            });
            assert!(failure.is_err());
            assert_eq!(
                read_node_full(&store, &id)
                    .unwrap()
                    .unwrap()
                    .body
                    .as_deref(),
                Some("beta second")
            );
            assert!(
                !fuzzy_recall(&store, "gamma", 10)
                    .unwrap()
                    .iter()
                    .any(|v| v.id == id)
            );
            assert!(Store::open_with_config(config.clone()).is_err());
            drop(store);
            let reopened = Store::open_with_config(config.clone()).unwrap();
            reopened.use_initiative("fixture");
            assert_eq!(
                read_node_full(&reopened, &id)
                    .unwrap()
                    .unwrap()
                    .body
                    .as_deref(),
                Some("beta second")
            );
            assert!(
                fuzzy_recall(&reopened, "beta", 10)
                    .unwrap()
                    .iter()
                    .any(|v| v.id == id)
            );
            forget(&reopened, &id).unwrap();
            assert!(read_node_full(&reopened, &id).unwrap().is_none());
            assert_eq!(
                at(&reopened, &id, first).unwrap().unwrap().body.as_deref(),
                Some("alpha first")
            );
        });
    }
}

#[test]
fn remote_backend_requires_namespace() {
    let config = KaeruConfig {
        storage: StorageConfig::Tikv {
            connection: TiKvConfig::default(),
        },
        ..KaeruConfig::defaults()
    };
    assert!(matches!(
        Store::open_with_config(config),
        Err(Error::Invalid(_))
    ));
}
