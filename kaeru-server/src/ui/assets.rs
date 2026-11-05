//! Embedded browser application; only compiled, allowlisted static assets.

use axum::{
    Router,
    extract::Request,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use include_dir::{Dir, include_dir};

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/ui-dist");

async fn asset(request: Request) -> Response {
    let path = request.uri().path();
    let file = match path {
        "/ui" | "/ui/" | "/ui/login" | "/ui/admin" => "index.html",
        _ => match path.strip_prefix("/ui/") {
            Some(path) => path,
            None => return StatusCode::NOT_FOUND.into_response(),
        },
    };
    if file.contains("..") || file.ends_with(".map") || file.ends_with("graph.json") {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Some(asset) = ASSETS.get_file(file) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let mime = match file.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "woff2" => "font/woff2",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        _ => return StatusCode::NOT_FOUND.into_response(),
    };
    let mut response = ([(header::CONTENT_TYPE, mime)], asset.contents()).into_response();
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        if file == "index.html" {
            "no-store"
        } else {
            "public, max-age=31536000, immutable"
        }
        .parse()
        .unwrap(),
    );
    response
        .headers_mut()
        .insert("x-content-type-options", "nosniff".parse().unwrap());
    response
        .headers_mut()
        .insert("x-frame-options", "DENY".parse().unwrap());
    response
        .headers_mut()
        .insert("referrer-policy", "no-referrer".parse().unwrap());
    response.headers_mut().insert("content-security-policy", "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; font-src 'self'; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'".parse().unwrap());
    response
}

pub fn router() -> Router {
    Router::new()
        .route("/ui", get(asset))
        .route("/ui/", get(asset))
        .route("/ui/{*path}", get(asset))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use tower::ServiceExt;

    #[tokio::test]
    async fn embedded_pages_and_fonts_work_without_a_source_checkout() {
        for path in ["/ui/", "/ui/login", "/ui/admin"] {
            let response = router()
                .oneshot(Request::get(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
            let body = to_bytes(response.into_body(), 65536).await.unwrap();
            assert!(
                String::from_utf8(body.to_vec())
                    .unwrap()
                    .contains("/ui/assets/")
            );
        }
        let fonts = ASSETS
            .get_dir("assets")
            .unwrap()
            .files()
            .filter(|file| file.path().extension().is_some_and(|e| e == "woff2"));
        let mut count = 0;
        for font in fonts {
            let response = router()
                .oneshot(
                    Request::get(format!("/ui/{}", font.path().display()))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.headers()[header::CONTENT_TYPE], "font/woff2");
            count += 1;
        }
        assert!(count > 0);
        for path in [
            "/ui/graph.json",
            "/graph.json",
            "/ui/manifest.json",
            "/ui/unknown",
            "/ui/assets/missing.js",
            "/ui/source.map",
        ] {
            assert_eq!(
                router()
                    .oneshot(Request::get(path).body(Body::empty()).unwrap())
                    .await
                    .unwrap()
                    .status(),
                StatusCode::NOT_FOUND
            );
        }
    }
}
