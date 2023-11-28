use axum::{Extension, Json};
use axum::http::HeaderMap;
use axum_macros::debug_handler;
use serde_json::json;

use libdatabase::{ConnectionTrait, DatabaseBackend, DbBatchQueryArgs, FromQueryResult, MySQL01, MySQLPool, sea_orm, TransactionTrait};
use libglobal_request_id::get_request_id;
use libtracing::info;

/// mysql 批量查询
#[debug_handler]
pub async fn mysql_query(
    headers: HeaderMap,
    Extension(mysql_01_pool): Extension<MySQLPool<MySQL01>>,
    Json(args): Json<DbBatchQueryArgs>,
) -> Result<Json<serde_json::Value>, String> {
    let request_id = get_request_id(&headers);
    info!("x-request-id={}", request_id);

    let sql_vec = args.str_sql_array;
    let conn = match args.str_node_env.as_str() {
        "prd" => mysql_01_pool,
        evn_name => Err(format!("未知的运行环境名称: {}", evn_name))?,
    };

    let mut rst_vec = vec![];
    for sql in sql_vec.into_iter() {
        let vec_colum_values = conn
            .query_all(sea_orm::Statement::from_string(DatabaseBackend::MySql, sql))
            .await
            .map_err(|err|format!("数据库查询失败: {}", err))?
            .into_iter()
            .map(|ref q1| sea_orm::query::JsonValue::from_query_result(q1, ""))
            .map(|j1| {
                if j1.is_ok() {
                    unsafe { j1.unwrap_unchecked() }
                } else {
                    sea_orm::JsonValue::String(format!("{:?}", j1))
                }
            })
            .collect::<sea_orm::JsonValue>();
        rst_vec.push(vec_colum_values);
    }

    Ok(Json(json!({
        "status": 0,
        "result": rst_vec,
    })))
}

/// mysql 批量事务
#[debug_handler]
pub async fn mysql_transaction(
    headers: HeaderMap,
    Extension(mysql_01_pool): Extension<MySQLPool<MySQL01>>,
    Json(args): Json<DbBatchQueryArgs>,
) -> Result<Json<serde_json::Value>, String> {
    let request_id = get_request_id(&headers);
    info!("x-request-id={}", request_id);

    let sql_vec = args.str_sql_array;
    let conn = match args.str_node_env.as_str() {
        "prd" => mysql_01_pool,
        evn_name => Err(format!("未知的运行环境名称: {}", evn_name))?,
    };

    // 开启事务
    let db_transaction = conn.begin().await.map_err(|err|format!("开启数据库事务失败: {}", err))?;

    let mut rst_vec = vec![];
    for sql in sql_vec.into_iter() {
        let exec_rst = db_transaction
            .execute(sea_orm::Statement::from_string(DatabaseBackend::MySql, sql))
            .await
            .map_err(|err|format!("数据库SQL执行失败: {}", err))?;
        rst_vec.push(json!({ "rows_affected":exec_rst.rows_affected(), "last_insert_id": exec_rst.last_insert_id()}));
    }

    // 结束事务
    db_transaction
        .commit()
        .await
        .map_err(|err|format!("数据库事务提交失败: {}", err))?;

    Ok(Json(json!({
        "status": 0,
        "result": rst_vec,
    })))
}
