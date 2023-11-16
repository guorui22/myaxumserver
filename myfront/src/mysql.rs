use std::collections::HashMap;
use std::marker::PhantomData;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

/// 自定义数据库连接池类型 MySQL01
#[derive(Clone, Debug)]
pub struct MySQL01;

/// 自定义数据库连接池
#[derive(Clone, Debug)]
pub struct MySQLPool<T> {
    pub db_conn: DatabaseConnection,
    _phantom: PhantomData<T>
}

/// 为结构体 MySQLPool<T>实现 new 方法
impl<T> MySQLPool<T> {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self {
            db_conn,
            _phantom: PhantomData::<T>
        }
    }
}

/// 为了实现 `Deref` trait，我们需要手动实现 `MySQL01DatabaseConnection` 的 `Deref` trait
impl<T> std::ops::Deref for MySQLPool<T> {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.db_conn
    }
}

// MySQL 数据库连接池初始化
pub async fn init_mysql_conn_pool(
    db_name: &str,
    param_map: &HashMap<String, String>,
) -> Result<DatabaseConnection, String> {
    let default = &String::default();
    let mut connect_opt = ConnectOptions::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        param_map.get("MY_USER").unwrap_or(default),
        param_map.get("MY_PWD").unwrap_or(default),
        param_map.get("MY_HOST").unwrap_or(default),
        param_map.get("MY_PORT").unwrap_or(default),
        param_map.get("MY_DB_NAME").unwrap_or(default)
    ));
    connect_opt
        .max_connections(64)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    let pool: DatabaseConnection = Database::connect(connect_opt).await.map_err(|err| {
        format!("MySQL 数据库连接池({}) is {}", db_name, err)
    })?;

    Ok(pool)
}
