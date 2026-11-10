//! Browser-mount boundaries over real account sessions and canonical reads.
use super::*;
use crate::settings::ui::UiMount;

fn fixture(prefix: &str) -> Fixture {
    let mut f = Fixture::new();
    f.auth.mount = UiMount::parse(prefix).unwrap();
    f
}
fn read_router(f: &Fixture) -> (Router, String) {
    let store = Arc::new(kaeru_core::Store::open_in_memory().unwrap());
    let result = kaeru_core::api::execute(
        &store,
        "note",
        json!({"initiative":"allowed","name":"synthetic","body":"Synthetic memory body."}),
        &kaeru_core::api::ApiOptions::local(),
    );
    let id = serde_json::to_value(result).unwrap()["data"]["primary"]["id"]
        .as_str()
        .unwrap()
        .to_string();
    let reads = crate::api::browser_read_router(
        store,
        crate::api::ApiConfig {
            allow: vec!["allowed".into()],
            include_local: true,
            ..Default::default()
        },
        crate::cloud_client::CloudRegistry::default(),
    )
    .layer(middleware::from_fn_with_state(
        f.auth.clone(),
        crate::ui::api_access,
    ));
    let mcp = Router::new()
        .route("/mcp", axum::routing::post(|| async { StatusCode::OK }))
        .layer(middleware::from_fn_with_state(
            Arc::<str>::from(f.auth.static_token.as_str()),
            crate::auth::require_bearer,
        ));
    (
        mcp.merge(crate::ui::router_with_reads(f.auth.clone(), Some(reads))),
        id,
    )
}
async fn login(app: Router, prefix: &str) -> (String, String) {
    let (status, headers, view) = call(
        app,
        "POST",
        &format!("{prefix}/auth/login"),
        json!({"username":"admin","password":PASSWORD}),
        &[("origin", "http://127.0.0.1:9876")],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{view}");
    let cookie = headers
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|v| v.to_str().unwrap())
        .find(|v| !v.contains("Max-Age=0"))
        .unwrap();
    assert!(cookie.contains(&format!("Path={prefix};")));
    assert!(cookie.contains("HttpOnly; SameSite=Strict"));
    (
        cookie.split(';').next().unwrap().into(),
        view["csrf_token"].as_str().unwrap().into(),
    )
}
fn cookies(headers: &HeaderMap) -> Vec<&str> {
    headers
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|v| v.to_str().unwrap())
        .collect()
}

#[tokio::test]
async fn browser_mount_serves_assets_deep_links_and_only_its_read_surface() {
    let f = fixture("/tools/memory");
    let (app, id) = read_router(&f);
    for path in [
        "/tools/memory/",
        "/tools/memory/login?notice=expired",
        "/tools/memory/admin",
    ] {
        let response = app
            .clone()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response.headers()["content-security-policy"]
                .to_str()
                .unwrap()
                .contains("base-uri 'none'")
        );
        let html = String::from_utf8(
            to_bytes(response.into_body(), 65536)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(html.contains("data-kaeru-ui-base=\"/tools/memory\""));
        assert!(html.contains("data-kaeru-api-base=\"/tools/memory\""));
        assert!(html.contains("/tools/memory/assets/"));
        assert!(!html.contains("/ui/assets/"));
    }
    let (status, headers, _) = call(
        app.clone(),
        "GET",
        "/tools/memory?view=board",
        Value::Null,
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(headers[header::LOCATION], "/tools/memory/?view=board");
    let args = json!({"initiative":"allowed","node":id});
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/tools/memory/v1/read",
            args.clone(),
            &[]
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let (cookie, csrf) = login(app.clone(), "/tools/memory").await;
    let headers = [
        ("cookie", cookie.as_str()),
        ("origin", "http://127.0.0.1:9876"),
        (web::CSRF_HEADER, csrf.as_str()),
    ];
    let (status, _, view) = call(
        app.clone(),
        "POST",
        "/tools/memory/v1/read",
        args.clone(),
        &headers,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{view}");
    assert_eq!(view["data"]["node"]["body"], "Synthetic memory body.");
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/tools/memory/v1/read",
            args.clone(),
            &[("cookie", &cookie), ("origin", "http://127.0.0.1:9876")]
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/tools/memory/v1/context",
            json!({"initiative":"allowed","view":"board"}),
            &headers
        )
        .await
        .0,
        StatusCode::OK
    );
    for path in [
        "/mcp",
        "/sse",
        "/messages",
        "/v1/note",
        "/v1/share",
        "/v1/cloud_recall",
        "/v1/export",
        "/v1/reflect",
        "/unknown",
    ] {
        for auth in [
            headers.to_vec(),
            vec![("authorization", "Bearer mcp-test-secret")],
        ] {
            assert_eq!(
                call(
                    app.clone(),
                    "POST",
                    &format!("/tools/memory{path}"),
                    json!({}),
                    &auth
                )
                .await
                .0,
                StatusCode::NOT_FOUND,
                "{path}"
            );
        }
    }
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/mcp",
            json!({}),
            &[("cookie", &cookie)]
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/mcp",
            json!({}),
            &[("authorization", "Bearer mcp-test-secret")]
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        call(
            app.clone(),
            "POST",
            "/tools/memory/auth/logout",
            json!({}),
            &headers
        )
        .await
        .0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        call(app.clone(), "POST", "/tools/memory/v1/read", args, &headers)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        call(
            app,
            "POST",
            "/mcp",
            json!({}),
            &[("authorization", "Bearer mcp-test-secret")]
        )
        .await
        .0,
        StatusCode::OK
    );
}
#[tokio::test]
async fn bootstrap_moves_only_single_valid_cookies_without_extending_the_jwt() {
    let f = fixture("/memory");
    let app = f.router();
    let (token, claims, _) = f.auth.session.issue(1).unwrap();
    let cookie = format!("{}={token}", web::SESSION_COOKIE);
    let (status, headers, view) = call(
        app.clone(),
        "GET",
        "/memory/auth/session",
        Value::Null,
        &[("cookie", &cookie)],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(view["authenticated"], true);
    assert!(cookies(&headers).is_empty());
    let (status, headers, view) = call(
        app.clone(),
        "POST",
        "/memory/auth/session",
        Value::Null,
        &[("cookie", &cookie), ("origin", "http://127.0.0.1:9876")],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(view["expires_at"], claims.exp);
    let set = cookies(&headers);
    assert_eq!(set.len(), 2);
    assert!(
        set.iter()
            .any(|v| v.contains("Path=/;") && v.contains("Max-Age=0"))
    );
    let scoped = set.iter().find(|v| v.contains("Path=/memory;")).unwrap();
    assert!(scoped.starts_with(&format!("{}={token};", web::SESSION_COOKIE)));
    assert!(!scoped.contains("; Secure"));
    let ttl = scoped
        .split("Max-Age=")
        .nth(1)
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    assert!(ttl > 0 && ttl <= claims.exp - claims.iat);
    for origin in [
        None,
        Some("null"),
        Some("https://foreign.test"),
        Some("http://127.0.0.1:9876/path"),
    ] {
        let mut hs = vec![("cookie", cookie.as_str())];
        if let Some(origin) = origin {
            hs.push(("origin", origin));
        }
        let (status, headers, _) = call(
            app.clone(),
            "POST",
            "/memory/auth/session",
            Value::Null,
            &hs,
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert!(cookies(&headers).is_empty());
    }
    let duplicate = format!("{cookie}; {cookie}");
    let (status, headers, view) = call(
        app.clone(),
        "POST",
        "/memory/auth/session",
        Value::Null,
        &[("cookie", &duplicate), ("origin", "http://127.0.0.1:9876")],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(view["authenticated"], false);
    assert_eq!(cookies(&headers).len(), 2);
    assert!(cookies(&headers).iter().all(|v| v.contains("Max-Age=0")));
    let bearer = format!("Bearer {token}");
    let (_, headers, view) = call(
        app.clone(),
        "POST",
        "/memory/auth/session",
        Value::Null,
        &[
            ("authorization", &bearer),
            ("origin", "http://127.0.0.1:9876"),
        ],
    )
    .await;
    assert_eq!(view["authenticated"], false);
    assert!(cookies(&headers).is_empty());
    let (_, headers, view) = call(
        app.clone(),
        "POST",
        "/memory/auth/session",
        Value::Null,
        &[
            ("cookie", "kaeru_session=invalid"),
            ("origin", "http://127.0.0.1:9876"),
        ],
    )
    .await;
    assert_eq!(view["authenticated"], false);
    assert!(cookies(&headers).iter().all(|v| v.contains("Max-Age=0")));
    let mut expired = claims.clone();
    let time = jsonwebtoken::get_current_timestamp();
    expired.iat = time - 120;
    expired.nbf = time - 120;
    expired.exp = time - 60;
    let signed = encode(
        &Header::new(Algorithm::HS256),
        &expired,
        &EncodingKey::from_secret(
            f.auth
                .session
                .account
                .as_ref()
                .unwrap()
                .snapshot()
                .jwt_secret
                .as_bytes(),
        ),
    )
    .unwrap();
    let expired_cookie = format!("kaeru_session={signed}");
    let (_, expired_headers, expired_view) = call(
        app.clone(),
        "POST",
        "/memory/auth/session",
        Value::Null,
        &[
            ("cookie", &expired_cookie),
            ("origin", "http://127.0.0.1:9876"),
        ],
    )
    .await;
    assert_eq!(expired_view["authenticated"], false);
    assert!(
        cookies(&expired_headers)
            .iter()
            .all(|v| v.contains("Max-Age=0"))
    );
    let (_, bearer_headers, bearer_view) = call(
        app.clone(),
        "POST",
        "/memory/auth/session",
        Value::Null,
        &[
            ("cookie", &cookie),
            ("authorization", "Bearer invalid"),
            ("origin", "http://127.0.0.1:9876"),
        ],
    )
    .await;
    assert_eq!(bearer_view["authenticated"], false);
    assert!(cookies(&bearer_headers).is_empty());
    login(app, "/memory").await;
}
#[tokio::test]
async fn bootstrap_cookie_secure_flag_uses_validated_origin_without_proxy_trust() {
    let f = fixture("/memory");
    let account = f.auth.session.account.as_ref().unwrap();
    account.update(1, |cfg| cfg.public_origin.clear()).unwrap();
    let (token, _, _) = f.auth.session.issue(2).unwrap();
    let cookie = format!("kaeru_session={token}");
    for (origin, secure) in [
        ("https://memory.example:8443", true),
        ("http://memory.example:8443", false),
    ] {
        let (status, headers, view) = call(
            f.router(),
            "POST",
            "/memory/auth/session",
            Value::Null,
            &[
                ("host", "memory.example:8443"),
                ("origin", origin),
                ("cookie", &cookie),
                ("x-forwarded-proto", if secure { "http" } else { "https" }),
            ],
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(view["authenticated"], true);
        assert!(
            cookies(&headers)
                .iter()
                .all(|v| v.contains("; Secure") == secure)
        );
    }
}
