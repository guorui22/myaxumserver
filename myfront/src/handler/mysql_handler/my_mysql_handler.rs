use std::collections::HashMap;

use axum::{Extension, Json};
use axum::http::HeaderMap;
use axum_macros::debug_handler;
use chrono::offset::Local;
use serde_json::json;

use libdatabase::{DbBatchQueryArgs, GrMySQLPool, sqlx, TestMySqlDb01};
use libdatabase::sqlx::{Column, Row, TypeInfo};
use libdatabase::sqlx::types::JsonValue;
use libglobal_request_id::get_request_id;
use libtracing::info;

/// mysql 批量查询
#[debug_handler]
pub async fn mysql_query(
    headers: HeaderMap,
    Extension(mysql_01_pool): Extension<GrMySQLPool<TestMySqlDb01>>,
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
        let vec_db_rows = sqlx::query(sql.as_str()).fetch_all(&*conn).await.map_err(|err| format!("数据库SQL执行失败: {}", err))?;
        let vec_db_rows_maps = vec_db_rows.iter().map(|current_row| {
            //列名集合
            let vec_row_columns_names = current_row.columns().iter().map(|c| c.name().to_string()).collect::<Vec<String>>();

            //列值集合
            let mut vec_row_columns_values = vec![];
            for current_column in current_row.columns() {
                macro_rules! get_mysql_column_value {
                    ($row:ident, $col:ident) => {
                        match $col.type_info().name() {
                            "VARBINARY" | "BINARY" | "BLOB" => {
                                $row.try_get::<Vec<u8>, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "VARCHAR" | "CHAR" | "TEXT" => {
                                $row.try_get::<String, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "DOUBLE" => {
                                $row.try_get::<f64, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "FLOAT" => {
                                $row.try_get::<f32, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "BIGINT UNSIGNED" => {
                                $row.try_get::<u64, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "INT UNSIGNED" => {
                                $row.try_get::<u32, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "SMALLINT UNSIGNED" => {
                                $row.try_get::<u16, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "TINYINT UNSIGNED" => {
                                $row.try_get::<u8, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "BIGINT" => {
                                $row.try_get::<i64, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "INT" => {
                                $row.try_get::<i32, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "SMALLINT" => {
                                $row.try_get::<i16, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "TINYINT" => {
                                $row.try_get::<i8, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "TINYINT(1)" | "BOOLEAN" | "BOOL" => {
                                $row.try_get::<bool, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "TIMESTAMP" => {
                                $row.try_get::<chrono::DateTime<Local>, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "DATETIME" => {
                                $row.try_get::<chrono::NaiveDateTime, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "DATE" => {
                                $row.try_get::<chrono::NaiveDate, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "TIME" => {
                                $row.try_get::<chrono::NaiveTime, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "DECIMAL" => {
                                $row.try_get::<bigdecimal::BigDecimal, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            "JSON" => {
                                $row.try_get::<serde_json::Value, _>($col.ordinal()).map(|s| json!(s)).unwrap_or_default()
                            }
                            _ => panic!("Unsupported type"),
                        }
                    };
                }
                let current_column_value = get_mysql_column_value!(current_row, current_column);
                vec_row_columns_values.push(current_column_value);
            }
            let current_row_hashmap: HashMap<String, JsonValue> = vec_row_columns_names.into_iter().zip(vec_row_columns_values.into_iter()).collect();
            current_row_hashmap
        }).collect::<Vec<HashMap<String, JsonValue>>>();

        rst_vec.push(vec_db_rows_maps);
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
    Extension(mysql_01_pool): Extension<GrMySQLPool<TestMySqlDb01>>,
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
    let mut db_transaction = conn.begin().await.map_err(|err| format!("开启数据库事务失败: {}", err))?;

    let mut rst_vec = vec![];
    for sql in sql_vec.into_iter() {
        let exec_rst = sqlx::query(sql.as_str())
            .execute(&mut *db_transaction)
            .await
            .map_err(|err| format!("数据库SQL执行失败: {}", err))?;
        rst_vec.push(json!({ "rows_affected":exec_rst.rows_affected(), "last_insert_id": exec_rst.last_insert_id()}));
    }

    // 结束事务
    db_transaction
        .commit()
        .await
        .map_err(|err| format!("数据库事务提交失败: {}", err))?;

    dbg!(&rst_vec);

    Ok(Json(json!({
        "status": 0,
        "result": rst_vec,
    })))
}
