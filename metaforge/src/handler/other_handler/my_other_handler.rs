use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::thread::current;
use std::time::Duration;
use axum::Extension;
use axum::http::{HeaderMap, StatusCode};
use axum_macros::debug_handler;
use libjsandbox::script::*;
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Receiver, Sender};
use libdatabase::sqlx::ColumnIndex;
use libdatabase::{GrMySQLPool, TestMySqlDb01};
use libjsandbox::script::Script;
use crate::MyArgs;

/// 测试函数
#[debug_handler]
pub async fn index(Extension(tx): Extension<Sender<MyArgs>>) -> String {

    let (sender, mut rxx) = tokio::sync::mpsc::channel::<String>(10);

    let msg = format!("Message {}", 0);
    tx.send(MyArgs { sender, msg }).await.unwrap();

    // 接收消息
    if let Some(msg) = rxx.recv().await {
        format!("Welcome to you! {} at {}", msg, chrono::Local::now())
    } else {
        format!("Welcome to you! at {}", chrono::Local::now())
    }

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
