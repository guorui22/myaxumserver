use axum::Extension;
use axum::http::{HeaderMap, StatusCode};
use axum_macros::debug_handler;
use tokio::sync::mpsc::Sender;

use crate::model::message::JsRsMsg;

/// 测试函数
#[debug_handler]
pub async fn index(Extension(_): Extension<Sender<JsRsMsg>>) -> String {
    format!("Welcome to you! at {}", chrono::Local::now())
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
