mod common;

use std::env;
use std::time::Duration;

use chrono::{Duration as ChronoDuration, Utc};
use contracts::{
    AccessCheck, Action, ActionMask, ApplyRightsRequest, BootstrapRequest, MembershipState,
    OrganizationRole, POLICY_VERSION, PrincipalState, Project, ProjectionChange, Resource,
    ResourceState, RevokeTokenRequest, SCHEMA_VERSION, TokenCeiling,
};
use kaeru_tokenizer_service::{cache::Cache, codec::decode, error::Error, storage::Store};
use redis::AsyncCommands;
use uuid::Uuid;

use common::database::{Database, authorize, create};
use common::{I1, I2, I3, OWNER_MEMBERSHIP, P1, P2, USER, USER_MEMBERSHIP, snapshot};

fn check(action: Action, id: Uuid) -> AccessCheck {
    AccessCheck {
        action,
        resource: Resource::Initiative(id),
    }
}

#[tokio::test]
#[ignore = "requires isolated PostgreSQL; see service README"]
async fn one_time_issuance_inheritance_moves_and_revocation() {
    let db = Database::new().await;
    db.bootstrap().await;
    let request = create();
    let result = db
        .store
        .create_token("control", request.clone())
        .await
        .unwrap();
    let secret = result.secret.unwrap();
    let replay = db
        .store
        .create_token("control", request.clone())
        .await
        .unwrap();
    assert_eq!(replay.token.id, result.token.id);
    assert!(replay.secret.is_none());
    assert!(replay.replayed);
    let status = db
        .store
        .token_operation("control", USER, request.operation_id)
        .await
        .unwrap();
    assert!(status.secret.is_none());
    assert!(
        authorize(
            &db.store,
            &secret,
            vec![check(Action::Read, I1), check(Action::Write, I1)]
        )
        .await
        .unwrap()
        .allowed
    );
    assert!(
        !authorize(
            &db.store,
            &secret,
            vec![check(Action::Read, I2), check(Action::Write, I2)]
        )
        .await
        .unwrap()
        .allowed
    );
    assert!(
        !authorize(&db.store, &secret, vec![check(Action::Read, I3)])
            .await
            .unwrap()
            .allowed
    );
    let mut member = snapshot()
        .memberships
        .into_iter()
        .find(|m| m.id == USER_MEMBERSHIP)
        .unwrap();
    member.all_projects = true;
    member.all_actions = ActionMask::of(&[Action::Read]);
    db.apply(vec![ProjectionChange::Membership(member.clone())])
        .await
        .unwrap();
    assert!(
        authorize(&db.store, &secret, vec![check(Action::Read, I3)])
            .await
            .unwrap()
            .allowed
    );
    assert!(
        !authorize(&db.store, &secret, vec![check(Action::Write, I3)])
            .await
            .unwrap()
            .allowed
    );
    let project = Project {
        id: Uuid::from_u128(100),
        organization_id: member.organization_id,
        state: ResourceState::Active,
    };
    let mut initiative = snapshot().initiatives[0].clone();
    initiative.id = Uuid::from_u128(101);
    initiative.project_id = Some(project.id);
    db.apply(vec![
        ProjectionChange::Initiative(initiative.clone()),
        ProjectionChange::Project(project),
    ])
    .await
    .unwrap();
    assert!(
        authorize(&db.store, &secret, vec![check(Action::Read, initiative.id)])
            .await
            .unwrap()
            .allowed
    );
    member.all_projects = false;
    db.apply(vec![ProjectionChange::Membership(member)])
        .await
        .unwrap();
    assert!(
        authorize(&db.store, &secret, vec![check(Action::Write, I1)])
            .await
            .unwrap()
            .allowed
    );
    assert!(
        !authorize(&db.store, &secret, vec![check(Action::Read, I3)])
            .await
            .unwrap()
            .allowed
    );

    let mut restricted = create();
    restricted.ceiling = TokenCeiling::Projects(vec![P1]);
    let restricted = db
        .store
        .create_token("control", restricted)
        .await
        .unwrap()
        .secret
        .unwrap();
    let mut moved = snapshot().initiatives[0].clone();
    moved.project_id = Some(P2);
    moved.placement_revision = 2;
    db.apply(vec![ProjectionChange::Initiative(moved.clone())])
        .await
        .unwrap();
    assert!(
        !authorize(&db.store, &restricted, vec![check(Action::Read, I1)])
            .await
            .unwrap()
            .allowed
    );
    moved.project_id = Some(P1);
    moved.placement_revision = 3;
    db.apply(vec![ProjectionChange::Initiative(moved)])
        .await
        .unwrap();
    assert!(
        authorize(&db.store, &secret, vec![check(Action::Read, I1)])
            .await
            .unwrap()
            .allowed
    );
    assert!(
        !authorize(&db.store, &secret, vec![check(Action::Write, I1)])
            .await
            .unwrap()
            .allowed
    );
    let revoke = RevokeTokenRequest {
        schema_version: SCHEMA_VERSION,
        operation_id: Uuid::now_v7(),
        actor_id: USER,
    };
    let revoked = db
        .store
        .revoke_token("control", result.token.id, revoke.clone())
        .await
        .unwrap();
    assert!(revoked.token.revoked_at.is_some());
    assert!(
        db.store
            .revoke_token("control", result.token.id, revoke)
            .await
            .unwrap()
            .replayed
    );
    assert!(matches!(
        authorize(&db.store, &secret, vec![check(Action::Read, I1)]).await,
        Err(Error::Unauthorized)
    ));
    let reopened = Store::connect(&db.url, 2, None, "reopened:test".into())
        .await
        .unwrap();
    assert!(matches!(
        authorize(&reopened, &secret, vec![check(Action::Read, I1)]).await,
        Err(Error::Unauthorized)
    ));
    reopened.close().await;
    db.cleanup().await;
}

#[tokio::test]
#[ignore = "requires isolated PostgreSQL; see service README"]
async fn concurrent_create_atomic_apply_and_membership_incarnations() {
    let db = Database::new().await;
    let bootstrap = BootstrapRequest {
        schema_version: SCHEMA_VERSION,
        policy_version: POLICY_VERSION,
        operation_id: Uuid::now_v7(),
        source_generation: Uuid::now_v7(),
        source_revision: 0,
        snapshot: snapshot(),
    };
    db.store.bootstrap(bootstrap.clone()).await.unwrap();
    let mut reordered = bootstrap;
    reordered.snapshot.principals.reverse();
    assert!(db.store.bootstrap(reordered).await.unwrap().replayed);
    let request = create();
    let (a, b) = tokio::join!(
        db.store.create_token("control", request.clone()),
        db.store.create_token("control", request.clone())
    );
    let a = a.unwrap();
    let b = b.unwrap();
    assert_eq!(a.token.id, b.token.id);
    assert_eq!(
        usize::from(a.secret.is_some()) + usize::from(b.secret.is_some()),
        1
    );
    assert_eq!(
        db.store
            .list_tokens(USER, None, 10)
            .await
            .unwrap()
            .tokens
            .len(),
        1
    );
    let secret = a.secret.or(b.secret).unwrap();
    let mut different = request;
    different.name = "Different operation content".into();
    assert!(matches!(
        db.store.create_token("control", different).await,
        Err(Error::Conflict(_))
    ));
    let before = db.store.state().await.unwrap();
    let gap = ApplyRightsRequest {
        schema_version: SCHEMA_VERSION,
        policy_version: POLICY_VERSION,
        operation_id: Uuid::now_v7(),
        source_generation: before.source_generation.unwrap(),
        expected_revision: 0,
        source_revision: 2,
        changes: vec![],
    };
    assert!(matches!(db.store.apply(gap).await, Err(Error::Conflict(_))));
    let mut invalid = snapshot().project_grants[0].clone();
    invalid.actions = ActionMask::of(&[Action::DeleteInitiative]);
    assert!(matches!(
        db.apply(vec![ProjectionChange::ProjectGrant(invalid)])
            .await,
        Err(Error::Invalid(_))
    ));
    assert_eq!(
        db.store.state().await.unwrap().authz_revision,
        before.authz_revision
    );

    let mut old = snapshot()
        .memberships
        .into_iter()
        .find(|m| m.id == USER_MEMBERSHIP)
        .unwrap();
    old.state = MembershipState::Left;
    let mut fresh = old.clone();
    fresh.id = Uuid::from_u128(99);
    fresh.state = MembershipState::Active;
    db.apply(vec![
        ProjectionChange::Membership(fresh.clone()),
        ProjectionChange::Membership(old.clone()),
    ])
    .await
    .unwrap();
    assert!(
        !authorize(&db.store, &secret, vec![check(Action::Read, I1)])
            .await
            .unwrap()
            .allowed
    );
    old.state = MembershipState::Active;
    assert!(matches!(
        db.apply(vec![ProjectionChange::Membership(old)]).await,
        Err(Error::Invalid(_))
    ));
    let mut owner = snapshot()
        .memberships
        .into_iter()
        .find(|m| m.id == OWNER_MEMBERSHIP)
        .unwrap();
    owner.role = OrganizationRole::Member;
    fresh.role = OrganizationRole::Admin;
    let mut org = snapshot().organizations.remove(0);
    org.owner_id = USER;
    db.apply(vec![
        ProjectionChange::Organization(org),
        ProjectionChange::Membership(fresh),
        ProjectionChange::Membership(owner),
    ])
    .await
    .unwrap();
    assert!(
        authorize(&db.store, &secret, vec![check(Action::Write, I3)])
            .await
            .unwrap()
            .allowed
    );
    let mut blocked = snapshot()
        .principals
        .into_iter()
        .find(|p| p.id == USER)
        .unwrap();
    blocked.state = PrincipalState::Blocked;
    db.apply(vec![ProjectionChange::Principal(blocked)])
        .await
        .unwrap();
    assert!(matches!(
        authorize(&db.store, &secret, vec![check(Action::Read, I3)]).await,
        Err(Error::Unauthorized)
    ));
    db.cleanup().await;
}

#[tokio::test]
#[ignore = "requires isolated PostgreSQL and Valkey; see service README"]
async fn shared_cache_namespace_cannot_mix_tokenizer_databases() {
    let url = env::var("TOKENIZER_TEST_VALKEY_URL").expect("set TOKENIZER_TEST_VALKEY_URL");
    let a = Database::new().await;
    let b = Database::new().await;
    let generation = Uuid::now_v7();
    for (db, grant) in [(&a, true), (&b, false)] {
        let mut projection = snapshot();
        if !grant {
            projection.project_grants.clear();
        }
        db.store
            .bootstrap(BootstrapRequest {
                schema_version: SCHEMA_VERSION,
                policy_version: POLICY_VERSION,
                operation_id: Uuid::now_v7(),
                source_generation: generation,
                source_revision: 0,
                snapshot: projection,
            })
            .await
            .unwrap();
    }
    let namespace = format!("shared:test:{}", Uuid::now_v7());
    let cache_a = Cache::connect(
        &url,
        Duration::from_secs(30),
        Duration::from_millis(100),
        1024 * 1024,
    )
    .await;
    let cache_b = Cache::connect(
        &url,
        Duration::from_secs(30),
        Duration::from_millis(100),
        1024 * 1024,
    )
    .await;
    let store_a = Store::connect(&a.url, 2, cache_a, namespace.clone())
        .await
        .unwrap();
    let store_b = Store::connect(&b.url, 2, cache_b, namespace).await.unwrap();
    let token_a = store_a
        .create_token("control", create())
        .await
        .unwrap()
        .secret
        .unwrap();
    let token_b = store_b
        .create_token("control", create())
        .await
        .unwrap()
        .secret
        .unwrap();
    assert_eq!(
        store_a.state().await.unwrap().authz_revision,
        store_b.state().await.unwrap().authz_revision
    );
    assert_ne!(
        store_a.state().await.unwrap().instance_id,
        store_b.state().await.unwrap().instance_id
    );
    assert!(
        authorize(&store_a, &token_a, vec![check(Action::Read, I1)])
            .await
            .unwrap()
            .allowed
    );
    assert!(
        !authorize(&store_b, &token_b, vec![check(Action::Read, I1)])
            .await
            .unwrap()
            .allowed
    );
    store_a.close().await;
    store_b.close().await;
    a.cleanup().await;
    b.cleanup().await;
}

#[tokio::test]
#[ignore = "requires isolated PostgreSQL and Valkey; see service README"]
async fn versioned_cache_cannot_restore_removed_permissions() {
    let url = env::var("TOKENIZER_TEST_VALKEY_URL")
        .expect("set TOKENIZER_TEST_VALKEY_URL to an isolated Valkey");
    let db = Database::new().await;
    db.bootstrap().await;
    let secret = db
        .store
        .create_token("control", create())
        .await
        .unwrap()
        .secret
        .unwrap();
    assert!(
        authorize(&db.store, &secret, vec![check(Action::Read, I1)])
            .await
            .unwrap()
            .allowed
    );
    let client = redis::Client::open(url).unwrap();
    let mut conn = client.get_multiplexed_async_connection().await.unwrap();
    let keys: Vec<String> = conn.keys(format!("{}:*", db.namespace)).await.unwrap();
    assert!(
        !keys.is_empty(),
        "authorization should populate derived facts cache"
    );
    let bytes: Vec<u8> = conn.get(&keys[0]).await.unwrap();
    let facts: contracts::ProjectionSnapshot = decode(&bytes, 32).unwrap();
    assert!(!facts.initiatives.is_empty());
    assert!(
        !bytes
            .windows(secret.expose().len())
            .any(|w| w == secret.expose().as_bytes())
    );
    db.apply(vec![ProjectionChange::RemoveProjectGrant {
        membership_id: USER_MEMBERSHIP,
        project_id: P1,
    }])
    .await
    .unwrap();
    let _: () = conn.set_ex(&keys[0], bytes, 30).await.unwrap();
    assert!(
        !authorize(&db.store, &secret, vec![check(Action::Read, I1)])
            .await
            .unwrap()
            .allowed
    );
    let keys: Vec<String> = conn.keys(format!("{}:*", db.namespace)).await.unwrap();
    let _: usize = conn.del(keys).await.unwrap();
    assert!(
        authorize(&db.store, &secret, vec![check(Action::Write, I1)])
            .await
            .unwrap()
            .allowed
    );
    let unavailable = Cache::connect(
        "redis://127.0.0.1:0",
        Duration::from_secs(1),
        Duration::from_millis(50),
        1024,
    )
    .await;
    assert!(unavailable.is_none());
    let no_cache = Store::connect(&db.url, 2, None, "fallback:test".into())
        .await
        .unwrap();
    assert!(
        authorize(&no_cache, &secret, vec![check(Action::Write, I1)])
            .await
            .unwrap()
            .allowed
    );
    no_cache.close().await;
    let mut expired = create();
    expired.expires_at = Some(Utc::now() + ChronoDuration::milliseconds(500));
    let token = db
        .store
        .create_token("control", expired)
        .await
        .unwrap()
        .secret
        .unwrap();
    tokio::time::sleep(Duration::from_millis(600)).await;
    assert!(matches!(
        authorize(&db.store, &token, vec![check(Action::Write, I1)]).await,
        Err(Error::Unauthorized)
    ));
    db.cleanup().await;
}
