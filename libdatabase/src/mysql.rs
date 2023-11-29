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
    _db_type: PhantomData<T>
}

/// 为结构体 GrMySQLPool<T>实现 new 方法
impl<T> GrMySQLPool<T> {
    pub fn new(db_conn: MySqlPool) -> Self {
        Self {
            db_conn,
            _db_type: PhantomData::<T>
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
    let mysql_pool: Pool<MySql> =  MySqlPoolOptions::new()
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
