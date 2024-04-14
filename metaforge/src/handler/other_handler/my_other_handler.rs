use std::path::Path;
use std::rc::Rc;
use std::thread::current;
use axum::http::{HeaderMap, StatusCode};
use axum_macros::debug_handler;

use serde::de::IntoDeserializer;
use libdatabase::sqlx::ColumnIndex;

/// 测试函数
#[debug_handler]
pub async fn index() -> String {

    // let mut script = jsandbox::Script::from_string("function add(a, b) { return a + b; }")
    //     .build()
    //     .unwrap();
    // let result: u32 = script.call("add", (1, 2)).await.unwrap();


    // format!("Welcome to you! at {}", rst)
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
