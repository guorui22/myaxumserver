use std::collections::HashMap;
use std::marker::PhantomData;
use std::time::Duration;
use sqlx::{MySql, MySqlPool, Pool};
use sqlx::mysql::MySqlPoolOptions;

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
) -> Result<GrMySQLPool<T>, String> {
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
                param_map.get("MY_USER").unwrap_or(def_val),
                param_map.get("MY_PWD").unwrap_or(def_val),
                param_map.get("MY_HOST").unwrap_or(def_val),
                param_map.get("MY_PORT").unwrap_or(def_val),
                param_map.get("MY_DB_NAME").unwrap_or(def_val),
            ).as_str()
        )
        .await
        .map_err(|err| err.to_string())?;

    Ok(GrMySQLPool::new(mysql_pool))
}

/// 宏定义：获取 MySQL 数据库 sqlx 查询结果中指定列的值
#[macro_export]
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
