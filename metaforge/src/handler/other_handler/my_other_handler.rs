use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::thread::current;
use std::time::Duration;
use axum::Extension;
use axum::http::{HeaderMap, StatusCode};
use axum_macros::debug_handler;
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Receiver, Sender};
use libdatabase::sqlx::ColumnIndex;
use libdatabase::{GrMySQLPool, TestMySqlDb01};
use libtracing::{debug, info};
use crate::model::message::JsRsMsg;

/// 测试函数
#[debug_handler]
pub async fn index(Extension(sender_main): Extension<Sender<JsRsMsg>>) -> String {
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
