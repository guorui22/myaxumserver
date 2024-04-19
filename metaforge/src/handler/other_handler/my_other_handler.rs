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
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Receiver, Sender};
use libdatabase::sqlx::ColumnIndex;
use libdatabase::{GrMySQLPool, TestMySqlDb01};
use libjsandbox::script::Script;

/// 测试函数
#[debug_handler]
pub async fn index(Extension(tx): Extension<Sender<String>>) -> String {

    // let mut script = jsandbox::Script::from_string("function add(a, b) { return a + b; }")
    //     .build()
    //     .unwrap();
    // let result: u32 = script.call("add", (1, 2)).await.unwrap();

    // 创建一个通道，用于线程间通信


    // 导入自定义函数
    // script.add_script(include_str!("output_01.js")).unwrap();

    // 调用自定义函数
    // let result: serde_json::Value = script.call("output_01.for_in_object", (serde_json::json!({"a1":1000, "a2": 2000}), )).await.unwrap();

    // 检查函数返回值
    // dbg!(&result.to_string());

    // format!("Welcome to you! at {}", rst)

    for i in 0..5 {
        let msg = format!("Message {}", i);
        tx.send(msg).await.unwrap();
    }

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
