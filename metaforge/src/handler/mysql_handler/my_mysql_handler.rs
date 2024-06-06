use axum::http::HeaderMap;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use tracing::info;

use libdatabase::{DbBatchQueryArgs, GrMySQLPool, TestMySqlDb01};
use crate::model::my_request_id::get_request_id;

/// mysql 批量查询
#[debug_handler]
pub async fn mysql_query(
    _headers: HeaderMap,
    Extension(_mysql_01_pool): Extension<GrMySQLPool<TestMySqlDb01>>,
    Json(args): Json<DbBatchQueryArgs>,
) -> Result<Json<serde_json::Value>, String> {
    let request_id = get_request_id(&_headers);
    info!("x-request-id={}", request_id);

    let sql_vec = args.str_sql_array;
    let conn = match args.str_node_env.as_str() {
        "prd" => _mysql_01_pool,
        evn_name => Err(format!("未知的运行环境名称: {}", evn_name))?,
    };

    libdatabase::mysql_query(conn, sql_vec)
        .await
        .map_err(|err| format!("数据库查询失败: {}", err))
        .map(Json)
}

/// mysql 批量事务
#[debug_handler]
pub async fn mysql_transaction(
    _headers: HeaderMap,
    Extension(_mysql_01_pool): Extension<GrMySQLPool<TestMySqlDb01>>,
    Json(args): Json<DbBatchQueryArgs>,
) -> Result<Json<serde_json::Value>, String> {
    let request_id = get_request_id(&_headers);
    info!("x-request-id={}", request_id);

    let sql_vec = args.str_sql_array;
    let conn = match args.str_node_env.as_str() {
        "prd" => _mysql_01_pool,
        evn_name => Err(format!("未知的运行环境名称: {}", evn_name))?,
    };

    libdatabase::mysql_transaction(conn, sql_vec, true)
        .await
        .map_err(|err| format!("数据库事务失败: {}", err))
        .map(Json)
}
