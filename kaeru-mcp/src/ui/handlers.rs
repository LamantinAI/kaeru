//! Browser authentication and administration over the existing instance API.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, patch, post},
};
use serde_json::json;

use crate::api::{ApiConfig, principal::Principal};
use crate::auth::session::WebSession;
use crate::auth::{bearer_token, constant_time_eq};
use crate::models::auth::{
    AccountInput, LoginInput, SessionClaims, SessionSettingsInput, SessionView,
};
use crate::settings::web;

#[derive(Clone)]
pub struct BrowserAuth {
    pub session: Arc<WebSession>,
    pub static_token: String,
    pub api: ApiConfig,
    pub clouds: Vec<String>,
}

fn denied(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({"error":message}))).into_response()
}

fn cookie_token(headers: &HeaderMap) -> Option<&str> {
    let mut matches = headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|value| value.trim().split_once('='))
        .filter(|(name, _)| *name == web::SESSION_COOKIE);
    let (_, value) = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    Some(value)
}

fn identity(state: &BrowserAuth, headers: &HeaderMap) -> Option<(SessionClaims, bool)> {
    if let Some(header) = headers.get(header::AUTHORIZATION) {
        let token = bearer_token(header)?;
        return state.session.verify(token).map(|claims| (claims, false));
    }
    state
        .session
        .verify(cookie_token(headers)?)
        .map(|claims| (claims, true))
}

fn request_origin(headers: &HeaderMap) -> Option<reqwest::Url> {
    let value = headers.get(header::ORIGIN)?.to_str().ok()?;
    let origin = reqwest::Url::parse(value).ok()?;
    if !matches!(origin.scheme(), "http" | "https")
        || origin.origin().ascii_serialization() != value
    {
        return None;
    }
    let host: axum::http::uri::Authority =
        headers.get(header::HOST)?.to_str().ok()?.parse().ok()?;
    let target = reqwest::Url::parse(&format!("{}://{host}", origin.scheme())).ok()?;
    (target.origin() == origin.origin()).then_some(origin)
}

fn origin_matches(state: &BrowserAuth, headers: &HeaderMap) -> bool {
    let Some(origin) = request_origin(headers) else {
        return false;
    };
    state.session.account.as_ref().is_some_and(|account| {
        let cfg = account.snapshot();
        cfg.public_origin.is_empty() || cfg.public_origin == origin.origin().ascii_serialization()
    })
}

fn csrf_matches(state: &BrowserAuth, headers: &HeaderMap, claims: &SessionClaims) -> bool {
    origin_matches(state, headers)
        && headers
            .get(web::CSRF_HEADER)
            .and_then(|h| h.to_str().ok())
            .is_some_and(|value| constant_time_eq(value.as_bytes(), claims.csrf.as_bytes()))
}

pub async fn api_access(
    State(state): State<BrowserAuth>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(token) = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(bearer_token)
        && !state.static_token.is_empty()
        && constant_time_eq(token.as_bytes(), state.static_token.as_bytes())
    {
        request.extensions_mut().insert(Principal::StaticBearer);
        return next.run(request).await;
    }
    if let Some((claims, cookie)) = identity(&state, request.headers()) {
        if cookie && !csrf_matches(&state, request.headers(), &claims) {
            return denied(
                StatusCode::FORBIDDEN,
                "Session origin or CSRF token is invalid",
            );
        }
        request.extensions_mut().insert(Principal::Web {
            username: claims.sub,
        });
        return next.run(request).await;
    }
    if state.static_token.is_empty() && state.session.account.is_none() {
        request.extensions_mut().insert(Principal::Local);
        return next.run(request).await;
    }
    denied(StatusCode::UNAUTHORIZED, "Sign in to continue")
}

async fn admin_access(
    State(state): State<BrowserAuth>,
    mut request: Request,
    next: Next,
) -> Response {
    let Some((claims, cookie)) = identity(&state, request.headers()) else {
        return denied(StatusCode::UNAUTHORIZED, "Sign in to continue");
    };
    if request.method() != axum::http::Method::GET
        && cookie
        && !csrf_matches(&state, request.headers(), &claims)
    {
        return denied(
            StatusCode::FORBIDDEN,
            "Session origin or CSRF token is invalid",
        );
    }
    request.extensions_mut().insert(claims);
    next.run(request).await
}

fn set_session_cookie(response: &mut Response, token: &str, ttl: u64, secure: bool) {
    let value = format!(
        "{}={token}; Path=/; HttpOnly; SameSite=Strict; Max-Age={ttl}{}",
        web::SESSION_COOKIE,
        if secure { "; Secure" } else { "" }
    );
    response.headers_mut().append(
        header::SET_COOKIE,
        value.parse().expect("JWT cookie uses URL-safe characters"),
    );
}

async fn login(
    State(state): State<BrowserAuth>,
    headers: HeaderMap,
    Json(input): Json<LoginInput>,
) -> Response {
    if state.session.account.is_none() {
        return denied(StatusCode::SERVICE_UNAVAILABLE, "Account not configured");
    }
    if !origin_matches(&state, &headers) {
        return denied(
            StatusCode::FORBIDDEN,
            "Sign in from the same address as this page",
        );
    }
    if !state.session.admit_login() {
        let mut response = denied(
            StatusCode::TOO_MANY_REQUESTS,
            "Too many attempts; try again in a minute",
        );
        response.headers_mut().insert(
            header::RETRY_AFTER,
            web::LOGIN_WINDOW_SECONDS.to_string().parse().unwrap(),
        );
        return response;
    }
    let account = state.session.account.as_ref().unwrap();
    let snapshot = account.snapshot();
    let password_ok = state
        .session
        .check_password(input.password, snapshot.revision)
        .await;
    if !password_ok
        || input.username != snapshot.username
        || account.snapshot().revision != snapshot.revision
    {
        return denied(StatusCode::UNAUTHORIZED, "Invalid username or password");
    }
    match state.session.issue(snapshot.revision) {
        Ok((token, claims, secure)) => {
            let mut response = Json(SessionView::authenticated(&claims)).into_response();
            set_session_cookie(
                &mut response,
                &token,
                claims.exp - claims.iat,
                secure || request_origin(&headers).is_some_and(|url| url.scheme() == "https"),
            );
            response
        }
        Err(_) => denied(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not create session",
        ),
    }
}

async fn session(State(state): State<BrowserAuth>, headers: HeaderMap) -> Response {
    let view = match identity(&state, &headers) {
        Some((claims, _)) => SessionView::authenticated(&claims),
        None => SessionView::signed_out(state.session.account.is_some()),
    };
    Json(view).into_response()
}

async fn logout(
    State(state): State<BrowserAuth>,
    axum::Extension(claims): axum::Extension<SessionClaims>,
) -> Response {
    let Some(account) = state.session.account.as_ref() else {
        return denied(StatusCode::UNAUTHORIZED, "Account not configured");
    };
    if account.update_async(claims.revision, |_| {}).await.is_err() {
        return denied(
            StatusCode::CONFLICT,
            "Session changed; reload before retrying",
        );
    }
    let mut response = StatusCode::NO_CONTENT.into_response();
    set_session_cookie(&mut response, "", 0, account.snapshot().secure_cookie());
    response
}

async fn settings(State(state): State<BrowserAuth>) -> Response {
    let account = state.session.account.as_ref().unwrap().snapshot();
    Json(json!({"version":env!("CARGO_PKG_VERSION"),"username":account.username,"ttl_seconds":account.ttl_seconds,
        "revision":account.revision,"public_origin":account.public_origin,"allow":state.api.allow,"deny":state.api.deny,
        "include_local":state.api.include_local,"mcp_static_auth":!state.static_token.is_empty(),"clouds":state.clouds})).into_response()
}

async fn update_account(
    State(state): State<BrowserAuth>,
    axum::Extension(claims): axum::Extension<SessionClaims>,
    Json(input): Json<AccountInput>,
) -> Response {
    if input.revision != claims.revision {
        return denied(
            StatusCode::CONFLICT,
            "Session changed; reload before retrying",
        );
    }
    if !state.session.admit_login() {
        return denied(
            StatusCode::TOO_MANY_REQUESTS,
            "Too many attempts; retry in a minute",
        );
    }
    if !state
        .session
        .check_password(input.current_password, claims.revision)
        .await
    {
        return denied(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Current password is incorrect",
        );
    }
    let hash = match input.new_password.filter(|value| !value.is_empty()) {
        Some(password) => match state.session.new_hash(password).await {
            Ok(hash) => Some(hash),
            Err(message) => return denied(StatusCode::BAD_REQUEST, &message),
        },
        None => None,
    };
    let account = state.session.account.as_ref().unwrap();
    match account
        .update_async(claims.revision, move |next| {
            next.username = input.username;
            if let Some(hash) = hash {
                next.password_hash = hash;
            }
        })
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => denied(
            StatusCode::CONFLICT,
            "Could not save account; reload settings and check the configuration file",
        ),
    }
}

async fn update_settings(
    State(state): State<BrowserAuth>,
    axum::Extension(claims): axum::Extension<SessionClaims>,
    Json(input): Json<SessionSettingsInput>,
) -> Response {
    if input.revision != claims.revision {
        return denied(
            StatusCode::CONFLICT,
            "Session changed; reload before retrying",
        );
    }
    if !state.session.admit_login() {
        return denied(
            StatusCode::TOO_MANY_REQUESTS,
            "Too many attempts; retry in a minute",
        );
    }
    if !state
        .session
        .check_password(input.current_password, claims.revision)
        .await
    {
        return denied(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Current password is incorrect",
        );
    }
    match state
        .session
        .account
        .as_ref()
        .unwrap()
        .update_async(claims.revision, move |next| {
            next.ttl_seconds = input.ttl_seconds
        })
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => denied(
            StatusCode::CONFLICT,
            "Could not save settings; lifetime must be 300–86400 seconds and the file writable",
        ),
    }
}

async fn no_store(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
    response
}

pub fn router(state: BrowserAuth) -> Router {
    let protected = Router::new()
        .route("/auth/logout", post(logout))
        .route("/admin/settings", get(settings).patch(update_settings))
        .route("/admin/account", patch(update_account))
        .route_layer(middleware::from_fn_with_state(state.clone(), admin_access));
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/session", get(session))
        .merge(protected)
        .layer(DefaultBodyLimit::max(web::MAX_REQUEST_BYTES))
        .layer(middleware::from_fn(no_store))
        .with_state(state)
}
