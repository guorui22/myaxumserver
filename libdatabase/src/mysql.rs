use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Column, MySql, MySqlPool, Pool, Row};
use std::collections::HashMap;
use std::env::args;
use std::marker::PhantomData;
use std::time::Duration;
use anyhow::anyhow;
use sea_orm::JsonValue;
use sea_orm::sea_query::extension::postgres::Extension;
use serde_json::json;
use sqlparser::ast::{Ident, ObjectName, Statement, TableFactor};
use sqlparser::dialect::{GenericDialect, MySqlDialect};
use sqlparser::parser::Parser;
use libtracing::info;
use crate::DbBatchQueryArgs;
use chrono::Local;
use sqlx::TypeInfo;

/// 自定义数据库连接池类型 TestMySqlDb01
#[derive(Clone, Debug)]
pub struct TestMySqlDb01;

/// 自定义数据库连接池
#[derive(Clone, Debug)]
pub struct GrMySQLPool<T> {
    pub db_conn: MySqlPool,
    _db_type: PhantomData<T>,
}

/// 为结构体 GrMySQLPool<T>实现 new 方法
impl<T> GrMySQLPool<T> {
    pub fn new(db_conn: MySqlPool) -> Self {
        Self {
            db_conn,
            _db_type: PhantomData::<T>,
        }
    }
}

/// 为了实现 `Deref` trait，我们需要手动实现 `GrMySQLPool<T>` 的 `Deref` trait
impl<T> std::ops::Deref for GrMySQLPool<T> {
    type Target = MySqlPool;

    fn deref(&self) -> &Self::Target {
        &self.db_conn
    }
}

// MySQL 数据库连接池初始化
pub async fn init_mysql_conn_pool<T>(
    param_map: &HashMap<String, String>,
) -> Result<GrMySQLPool<T>, anyhow::Error> {
    let def_val = &"".to_string();
    let mysql_pool: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(32)
        .min_connections(2)
        .idle_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(60 * 60))
        .connect(
            format!(
                "mysql://{}:{}@{}:{}/{}",
                param_map.get("my_user").unwrap_or(def_val),
                param_map.get("my_pwd").unwrap_or(def_val),
                param_map.get("my_host").unwrap_or(def_val),
                param_map.get("my_port").unwrap_or(def_val),
                param_map.get("my_db_name").unwrap_or(def_val),
            )
                .as_str(),
        )
        .await
        .map_err(|err| anyhow!(err))?;

    Ok(GrMySQLPool::new(mysql_pool))
}

/// 宏定义：获取 MySQL 数据库 sqlx 查询结果中指定列的值
#[macro_export]
macro_rules! get_mysql_column_value {
    ($row:ident, $col:ident) => {
        match $col.type_info().name() {
            "VARBINARY" | "BINARY" | "BLOB" => $row
                .try_get::<Vec<u8>, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "VARCHAR" | "CHAR" | "TEXT" => $row
                .try_get::<String, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "DOUBLE" => $row
                .try_get::<f64, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "FLOAT" => $row
                .try_get::<f32, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "BIGINT UNSIGNED" => $row
                .try_get::<u64, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "INT UNSIGNED" => $row
                .try_get::<u32, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "SMALLINT UNSIGNED" => $row
                .try_get::<u16, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "TINYINT UNSIGNED" => $row
                .try_get::<u8, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "BIGINT" => $row
                .try_get::<i64, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "INT" => $row
                .try_get::<i32, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "SMALLINT" => $row
                .try_get::<i16, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "TINYINT" => $row
                .try_get::<i8, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "TINYINT(1)" | "BOOLEAN" | "BOOL" => $row
                .try_get::<bool, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "TIMESTAMP" => $row
                .try_get::<chrono::DateTime<Local>, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "DATETIME" => $row
                .try_get::<chrono::NaiveDateTime, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "DATE" => $row
                .try_get::<chrono::NaiveDate, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "TIME" => $row
                .try_get::<chrono::NaiveTime, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "DECIMAL" => $row
                .try_get::<bigdecimal::BigDecimal, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            "JSON" => $row
                .try_get::<serde_json::Value, _>($col.ordinal())
                .map(|s| json!(s))
                .unwrap_or_default(),
            _ => panic!("Unsupported type"),
        }
    };
}

/// mysql 批量查询
pub async fn mysql_query<T>(conn: GrMySQLPool<T>, sql_vec: Vec<String>) -> Result<serde_json::Value, anyhow::Error> {
    let mut rst_vec = vec![];
    for sql in sql_vec.into_iter() {
        let vec_db_rows = sqlx::query(sql.as_str())
            .fetch_all(&*conn)
            .await
            .map_err(|err| anyhow!("数据库SQL执行失败: {}", err))?;
        let vec_db_rows_maps = vec_db_rows
            .iter()
            .map(|current_row| {
                //列名集合
                let vec_row_columns_names = current_row
                    .columns()
                    .iter()
                    .map(|c| c.name().to_string())
                    .collect::<Vec<String>>();

                //列值集合
                let mut vec_row_columns_values = vec![];
                for current_column in current_row.columns() {
                    let current_column_value = get_mysql_column_value!(current_row, current_column);
                    vec_row_columns_values.push(current_column_value);
                }
                let current_row_hashmap: HashMap<String, JsonValue> = vec_row_columns_names
                    .into_iter()
                    .zip(vec_row_columns_values.into_iter())
                    .collect();
                current_row_hashmap
            })
            .collect::<Vec<HashMap<String, JsonValue>>>();

        rst_vec.push(vec_db_rows_maps);
    }

    Ok(json!({
        "status": 0,
        "result": rst_vec,
    }))
}

/// mysql 批量事务
/// conn: 数据库连接池
/// sql_vec: SQL 语句集合
/// is_ttl: 是否开启TTL
pub async fn mysql_transaction<T>(conn: GrMySQLPool<T>, sql_vec: Vec<String>, is_ttl: bool) -> Result<serde_json::Value, anyhow::Error> {

    // 开启事务
    let mut db_transaction = conn
        .begin()
        .await
        .map_err(|err| anyhow!("开启数据库事务失败: {}", err))?;

    let mut rst_vec = vec![];
    for sql in sql_vec.into_iter() {
        let exec_rst = sqlx::query(sql.clone().as_str())
            .execute(&mut *db_transaction)
            .await
            .map_err(|err| anyhow!("数据库SQL执行失败: {}", err))?;
        rst_vec.push(json!({ "rows_affected":exec_rst.rows_affected(), "last_insert_id": exec_rst.last_insert_id()}));

        // 如果开启TTL，则执行TTL SQL，记录数据的完整生命周期
        if is_ttl {
            let ttl_sql = mysql_sql_ttl(sql.clone()).await?;
            let exec_rst_ttl = sqlx::query(ttl_sql.as_str())
                .execute(&mut *db_transaction)
                .await
                .map_err(|err| anyhow!("数据库TTL SQL执行失败: {}", err))?;
        }
    }

    // 结束事务
    db_transaction
        .commit()
        .await
        .map_err(|err| anyhow!("数据库事务提交失败: {}", err))?;

    dbg!(&rst_vec);

    Ok(json!({
        "status": 0,
        "result": rst_vec,
    }))
}

/// mysql 全生命周期 SQL 语句生成
pub async fn mysql_sql_ttl(sql: String) -> Result<String, anyhow::Error> {

    let mut binding = Parser::parse_sql(&MySqlDialect {}, sql.as_str())
        .map_err(|err| anyhow!("SQL解析失败: {}", err))?;
    let mut statement = binding.into_iter().next().ok_or_else(|| anyhow!("SQL解析失败"))?;

    // 检查语句是否是INSERT类型
    if let Statement::Insert(ref mut insert) = statement {
        let new_table_name = format!("{}_ttl", insert.table_name);
        insert.table_name = ObjectName(vec![Ident { value: new_table_name, quote_style: None }]);
        // 将修改后的AST转换回SQL字符串
        let ttl_insert_sql = format!("{}", statement);
        Ok(ttl_insert_sql)
    }
    // 检查语句是否是UPDATE类型
    else if let Statement::Update { ref table, assignments, from, ref selection, returning } = statement {
        let table_names = match &table.relation {
            TableFactor::Table { name, .. } => {
                let new_table_name = format!("{}_ttl", name);
                (name.to_string(), new_table_name)
            }
            _ => {
                return Err(anyhow!("Table name not found"));
            }
        };
        let ttl_insert_sql = if let Some(ref expr) = selection {
            format!("INSERT INTO {} SELECT * FROM {} {}", table_names.1, table_names.0, format!("WHERE {}", expr.to_string()))
        } else {
            format!("INSERT INTO {} SELECT * FROM {}", table_names.1, table_names.0)
        };
        Ok(ttl_insert_sql)
    } else {
        Err(anyhow!("SQL解析失败"))
    }
}
