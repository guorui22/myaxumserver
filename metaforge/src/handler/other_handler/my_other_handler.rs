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
use libtracing::{debug, info};
use crate::model::message::JsRsMsg;

/// 测试函数
#[debug_handler]
pub async fn index(Extension(sender_main): Extension<Sender<JsRsMsg>>) -> String {
    let (sender_js, mut receiver_main) = tokio::sync::oneshot::channel::<serde_json::Value>();

    let msg = format!("Message {}", 0);
    sender_main.send(
        JsRsMsg {
            sender: sender_js,
            js_name: String::from("output_01"),
            js_method_name: String::from("for_in_object"),
            js_method_args: serde_json::json!({"a1":1000, "a2": 2000})
        }
    ).await.unwrap();

    // 接收消息
    if let Ok(msg) = receiver_main.await {
        info!("{}",&msg);
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
