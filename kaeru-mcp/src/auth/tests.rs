use std::sync::Arc;

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{HeaderMap, Request, StatusCode, header},
    middleware,
};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde_json::{Value, json};
use tower::ServiceExt;

use super::{
    session::{WebSession, hash_password},
    store::AccountStore,
};
use crate::{
    settings::web::{self, WebConfig},
    ui::BrowserAuth,
};

const PASSWORD: &str = "correct horse battery staple";

fn config() -> WebConfig {
    WebConfig {
        username: "admin".into(),
        password_hash: hash_password(PASSWORD).unwrap(),
        jwt_secret: "ab".repeat(32),
        public_origin: "http://127.0.0.1:9876".into(),
        ttl_seconds: 3600,
        issuer: web::SESSION_ISSUER.into(),
        audience: web::SESSION_AUDIENCE.into(),
        revision: 1,
    }
}

struct Fixture {
    directory: tempfile::TempDir,
    auth: BrowserAuth,
}

impl Fixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("web.toml");
        AccountStore::initialize(path.clone(), config()).unwrap();
        let session = WebSession::new(AccountStore::open(path).unwrap());
        Self {
            directory,
            auth: BrowserAuth {
                session,
                static_token: "mcp-test-secret".into(),
                api: crate::api::ApiConfig {
                    allow: vec!["allowed".into()],
                    ..Default::default()
                },
                clouds: vec![],
            },
        }
    }
    fn router(&self) -> Router {
        crate::ui::router(self.auth.clone())
    }
}

async fn call(
    app: Router,
    method: &str,
    path: &str,
    body: Value,
    headers: &[(&str, &str)],
) -> (StatusCode, HeaderMap, Value) {
    let mut request = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json");
    for (name, value) in headers {
        request = request.header(*name, *value);
    }
    if !headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("host"))
    {
        request = request.header(header::HOST, "127.0.0.1:9876");
    }
    let response = app
        .oneshot(request.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = to_bytes(response.into_body(), 65536).await.unwrap();
    let value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, headers, value)
}

async fn sign_in(app: Router) -> (String, String) {
    let (status, headers, body) = call(
        app,
        "POST",
        "/auth/login",
        json!({"username":"admin","password":PASSWORD}),
        &[("origin", "http://127.0.0.1:9876")],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let cookie = headers[header::SET_COOKIE].to_str().unwrap();
    assert!(
        cookie.contains("HttpOnly")
            && cookie.contains("SameSite=Strict")
            && cookie.contains("Path=/")
    );
    assert!(!body.to_string().contains("eyJ"));
    (
        cookie.split(';').next().unwrap().into(),
        body["csrf_token"].as_str().unwrap().into(),
    )
}

#[test]
fn jwt_validation_rejects_wrong_claims_algorithms_signatures_and_missing_fields() {
    let fixture = Fixture::new();
    let session = &fixture.auth.session;
    let (token, original, _) = session.issue(1).unwrap();
    assert_eq!(session.verify(&token).unwrap().sub, "admin");
    for variant in 0..8 {
        let mut claims = original.clone();
        let mut algorithm = Algorithm::HS256;
        let mut secret = "ab".repeat(32);
        match variant {
            0 => claims.exp = claims.iat - 1,
            1 => claims.nbf += 1000,
            2 => claims.iss = "foreign".into(),
            3 => claims.aud = "foreign".into(),
            4 => claims.sub = "another-user".into(),
            5 => claims.revision += 1,
            6 => algorithm = Algorithm::HS512,
            7 => secret = "cd".repeat(32),
            _ => unreachable!(),
        }
        let forged = encode(
            &Header::new(algorithm),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();
        assert!(session.verify(&forged).is_none(), "variant {variant}");
    }
    for field in ["exp", "iat", "nbf", "iss", "aud", "sub", "revision", "csrf"] {
        let mut claims = serde_json::to_value(&original).unwrap();
        claims.as_object_mut().unwrap().remove(field);
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret("ab".repeat(32).as_bytes()),
        )
        .unwrap();
        assert!(session.verify(&token).is_none(), "missing {field}");
    }
    assert!(session.issue(0).is_err());
}

#[test]
fn configuration_persists_sessions_and_revocation_across_restarts() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("web.toml");
    AccountStore::initialize(path.clone(), config()).unwrap();
    let session = WebSession::new(AccountStore::open(path.clone()).unwrap());
    let (token, _, _) = session.issue(1).unwrap();
    drop(session);
    let session = WebSession::new(AccountStore::open(path.clone()).unwrap());
    assert!(session.verify(&token).is_some());
    session.account.as_ref().unwrap().update(1, |_| {}).unwrap();
    drop(session);
    let session = WebSession::new(AccountStore::open(path.clone()).unwrap());
    assert!(session.verify(&token).is_none());
    assert!(
        std::fs::read_to_string(path)
            .unwrap()
            .contains("# basic-creds: admin:admin")
    );
}

#[test]
fn failed_config_write_and_revision_conflict_leave_credentials_unchanged() {
    let fixture = Fixture::new();
    let account = fixture.auth.session.account.as_ref().unwrap();
    assert!(
        AccountStore::open(fixture.directory.path().join("web.toml")).is_err(),
        "only one writer can open the account"
    );
    assert!(
        account
            .update(0, |next| next.username = "changed".into())
            .is_err()
    );
    let moved = fixture.directory.path().with_extension("moved");
    std::fs::rename(fixture.directory.path(), &moved).unwrap();
    assert!(
        account
            .update(1, |next| next.username = "changed".into())
            .is_err()
    );
    assert_eq!(account.snapshot().username, "admin");
    assert_eq!(account.snapshot().revision, 1);
    std::fs::rename(moved, fixture.directory.path()).unwrap();
}

#[tokio::test]
async fn login_is_rate_limited_and_rejects_foreign_origin() {
    let fixture = Fixture::new();
    let app = fixture.router();
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/auth/login",
            json!({"username":"admin","password":PASSWORD}),
            &[("origin", "https://foreign.test")]
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    for _ in 0..web::LOGIN_ATTEMPTS {
        assert_eq!(
            call(
                app.clone(),
                "POST",
                "/auth/login",
                json!({"username":"unknown","password":"incorrect password"}),
                &[("origin", "http://127.0.0.1:9876")]
            )
            .await
            .0,
            StatusCode::UNAUTHORIZED
        );
    }
    let (status, headers, _) = call(
        app,
        "POST",
        "/auth/login",
        json!({"username":"admin","password":PASSWORD}),
        &[("origin", "http://127.0.0.1:9876")],
    )
    .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert!(headers.contains_key(header::RETRY_AFTER));
}

#[tokio::test]
async fn account_update_and_logout_revoke_sessions_and_keep_admin_secrets_private() {
    let fixture = Fixture::new();
    let app = fixture.router();
    let (cookie, csrf) = sign_in(app.clone()).await;
    let headers = [
        ("cookie", cookie.as_str()),
        ("origin", "http://127.0.0.1:9876"),
        (web::CSRF_HEADER, csrf.as_str()),
    ];
    let (status, _, settings) =
        call(app.clone(), "GET", "/admin/settings", Value::Null, &headers).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        !settings.to_string().contains("password_hash")
            && !settings.to_string().contains("jwt_secret")
    );
    assert_eq!(call(app.clone(), "PATCH", "/admin/account", json!({"username":"new-admin","current_password":PASSWORD,"new_password":"another correct password","revision":1}), &[("cookie",cookie.as_str())]).await.0, StatusCode::FORBIDDEN);
    assert_eq!(call(app.clone(), "PATCH", "/admin/account", json!({"username":"new-admin","current_password":PASSWORD,"new_password":"another correct password","revision":1}), &headers).await.0, StatusCode::NO_CONTENT);
    assert_eq!(
        call(app.clone(), "GET", "/admin/settings", Value::Null, &headers)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        fixture
            .auth
            .session
            .account
            .as_ref()
            .unwrap()
            .snapshot()
            .username,
        "new-admin"
    );
    let raw = std::fs::read_to_string(fixture.directory.path().join("web.toml")).unwrap();
    assert!(!raw.contains(PASSWORD) && !raw.contains("another correct password"));
    let (token, claims, _) = fixture.auth.session.issue(2).unwrap();
    let cookie = format!("{}={token}", web::SESSION_COOKIE);
    let headers = [
        ("cookie", cookie.as_str()),
        ("origin", "http://127.0.0.1:9876"),
        (web::CSRF_HEADER, claims.csrf.as_str()),
    ];
    assert_eq!(
        call(app.clone(), "POST", "/auth/logout", Value::Null, &headers)
            .await
            .0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        call(app, "GET", "/admin/settings", Value::Null, &headers)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn jwt_api_preserves_scope_visibility_and_static_mcp_auth() {
    let fixture = Fixture::new();
    let store = Arc::new(kaeru_core::Store::open_in_memory().unwrap());
    let api = crate::api::router_with_clouds(
        store.clone(),
        fixture.auth.api.clone(),
        crate::cloud_client::CloudRegistry::default(),
    )
    .layer(middleware::from_fn_with_state(
        fixture.auth.clone(),
        crate::ui::api_access,
    ));
    let mcp = Router::new()
        .route("/mcp", axum::routing::get(|| async { "mcp" }))
        .layer(middleware::from_fn_with_state(
            Arc::<str>::from("mcp-test-secret"),
            super::require_bearer,
        ));
    let app = fixture.router().merge(api).merge(mcp);
    let (cookie, csrf) = sign_in(app.clone()).await;
    assert_eq!(
        call(app.clone(), "POST", "/v1/initiatives", json!({}), &[])
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/v1/initiatives",
            json!({}),
            &[("cookie", cookie.as_str())]
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    let headers = [
        ("cookie", cookie.as_str()),
        ("origin", "http://127.0.0.1:9876"),
        (web::CSRF_HEADER, csrf.as_str()),
    ];
    assert_eq!(
        call(app.clone(), "POST", "/v1/initiatives", json!({}), &headers)
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/v1/context",
            json!({"initiative":"forbidden"}),
            &headers
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let local = kaeru_core::api::execute(
        &store,
        "note",
        json!({"initiative":"allowed","body":"private content"}),
        &kaeru_core::api::ApiOptions::local(),
    );
    let id = match local.data {
        kaeru_core::api::ResponseData::Mutation(data) => data.primary.unwrap().id,
        _ => panic!(),
    };
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/v1/read",
            json!({"initiative":"allowed","node":id}),
            &headers
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        call(app.clone(), "GET", "/mcp", Value::Null, &headers)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        call(
            app.clone(),
            "GET",
            "/mcp",
            Value::Null,
            &[("authorization", "Bearer mcp-test-secret")]
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        call(
            app.clone(),
            "GET",
            "/admin/settings",
            Value::Null,
            &[("authorization", "Bearer mcp-test-secret")]
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        call(app.clone(), "POST", "/auth/logout", Value::Null, &headers)
            .await
            .0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        call(
            app.clone(),
            "GET",
            "/mcp",
            Value::Null,
            &[("authorization", "Bearer mcp-test-secret")]
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        call(app, "POST", "/v1/initiatives", json!({}), &headers)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn unconfigured_ui_has_no_default_account_or_admin_access() {
    let state = BrowserAuth {
        session: WebSession::new(None),
        static_token: String::new(),
        api: Default::default(),
        clouds: vec![],
    };
    let app = crate::ui::router(state);
    let (_, _, session) = call(app.clone(), "GET", "/auth/session", Value::Null, &[]).await;
    assert_eq!(session["configured"], false);
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/auth/login",
            json!({"username":"admin","password":"admin"}),
            &[("origin", "http://127.0.0.1:9876")]
        )
        .await
        .0,
        StatusCode::SERVICE_UNAVAILABLE
    );
    assert_eq!(
        call(app, "GET", "/admin/settings", Value::Null, &[])
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn browser_login_uses_request_host_and_rejects_cross_origin_requests() {
    let fixture = Fixture::new();
    let account = fixture.auth.session.account.as_ref().unwrap();
    let password_hash = hash_password("admin").unwrap();
    account
        .update(1, |cfg| {
            cfg.password_hash = password_hash;
            cfg.public_origin.clear();
        })
        .unwrap();
    let app = fixture.router();
    for (host, origin, secure) in [
        ("127.0.0.1:9877", "http://127.0.0.1:9877", false),
        ("192.168.0.166:9877", "http://192.168.0.166:9877", false),
        ("memory.example", "https://memory.example", true),
        ("[::1]:9877", "http://[::1]:9877", false),
    ] {
        let headers = [("host", host), ("origin", origin)];
        let (status, response_headers, view) = call(
            app.clone(),
            "POST",
            "/auth/login",
            json!({"username":"admin", "password":"admin"}),
            &headers,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{origin}");
        let cookie = response_headers[header::SET_COOKIE].to_str().unwrap();
        assert_eq!(cookie.contains("; Secure"), secure);
        let cookie = cookie.split(';').next().unwrap();
        let csrf = view["csrf_token"].as_str().unwrap();
        assert_eq!(
            call(
                app.clone(),
                "GET",
                "/admin/settings",
                Value::Null,
                &[("host", host), ("cookie", cookie)]
            )
            .await
            .0,
            StatusCode::OK
        );
        assert_eq!(
            call(
                app.clone(),
                "POST",
                "/auth/logout",
                Value::Null,
                &[
                    ("host", host),
                    ("origin", "https://foreign.test"),
                    ("cookie", cookie),
                    (web::CSRF_HEADER, csrf)
                ]
            )
            .await
            .0,
            StatusCode::FORBIDDEN
        );
    }
    for origin in [
        "https://foreign.test",
        "null",
        "http://127.0.0.1:9877",
        "http://192.168.0.166:9877/path",
    ] {
        assert_eq!(
            call(
                app.clone(),
                "POST",
                "/auth/login",
                json!({"username":"admin", "password":"admin"}),
                &[("host", "192.168.0.166:9877"), ("origin", origin)]
            )
            .await
            .0,
            StatusCode::FORBIDDEN
        );
    }
    assert!(hash_password("").is_err());
}

#[tokio::test]
async fn an_inflight_admin_request_cannot_use_a_future_revision_after_logout() {
    use axum::body::Bytes;
    use tokio::sync::oneshot;
    use tokio_stream::StreamExt;

    for path in ["/admin/account", "/admin/settings"] {
        let fixture = Fixture::new();
        let app = fixture.router();
        let (cookie, csrf) = sign_in(app.clone()).await;
        let payload =
            json!({"username":"must-not-change", "current_password":PASSWORD, "revision":2});
        let payload = if path == "/admin/settings" {
            json!({"current_password":PASSWORD, "ttl_seconds":600, "revision":2})
        } else {
            payload
        };
        let (started_tx, started_rx) = oneshot::channel();
        let (release_tx, release_rx) = oneshot::channel();
        let mut gate = Some((started_tx, release_rx, payload));
        let body = Body::from_stream(tokio_stream::once(()).then(move |()| {
            let (started_tx, release_rx, payload) = gate.take().unwrap();
            async move {
                // Extraction starts after authentication accepted revision 1.
                started_tx.send(()).unwrap();
                release_rx.await.unwrap();
                Ok::<_, std::io::Error>(Bytes::from(payload.to_string()))
            }
        }));
        let request = Request::builder()
            .method("PATCH")
            .uri(path)
            .header("content-type", "application/json")
            .header("host", "127.0.0.1:9876")
            .header("origin", "http://127.0.0.1:9876")
            .header("cookie", &cookie)
            .header(web::CSRF_HEADER, &csrf)
            .body(body)
            .unwrap();
        let pending = tokio::spawn(app.clone().oneshot(request));
        tokio::time::timeout(std::time::Duration::from_secs(2), started_rx)
            .await
            .unwrap()
            .unwrap();
        let headers = [
            ("cookie", cookie.as_str()),
            ("origin", "http://127.0.0.1:9876"),
            (web::CSRF_HEADER, csrf.as_str()),
        ];
        assert_eq!(
            call(app, "POST", "/auth/logout", Value::Null, &headers)
                .await
                .0,
            StatusCode::NO_CONTENT
        );
        release_tx.send(()).unwrap();
        assert_eq!(
            pending.await.unwrap().unwrap().status(),
            StatusCode::CONFLICT
        );
        let cfg = fixture.auth.session.account.as_ref().unwrap().snapshot();
        assert_eq!(cfg.revision, 2);
        assert_eq!(cfg.username, "admin");
        assert_eq!(cfg.ttl_seconds, 3600);
        assert!(
            !fixture
                .auth
                .session
                .check_password(PASSWORD.into(), 1)
                .await
        );
    }
}
