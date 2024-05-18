use axum::http::{HeaderMap, StatusCode};
use axum_macros::debug_handler;
use crate::auth::Claims;

/// 测试函数
#[debug_handler]
pub async fn index(claims: Claims) -> String {
    format!("Welcome to you! at {} for {}", chrono::Local::now(), claims.name)
}

/// 本站页面跳转
#[debug_handler]
pub async fn redirect01() -> (StatusCode, HeaderMap, ()) {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::LOCATION,
        "/templates/redirect.html".parse().unwrap(),
    );
    (StatusCode::FOUND, headers, ())
}

/// 外站页面跳转
#[debug_handler]
pub async fn redirect02() -> (StatusCode, HeaderMap, ()) {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::LOCATION,
        "https://www.baidu.com".parse().unwrap(),
    );
    (StatusCode::FOUND, headers, ())
}
