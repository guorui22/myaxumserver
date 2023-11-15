use std::collections::HashMap;
use std::time::Duration;

use anyhow::{anyhow, Error};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

/// 自定义数据库连接池类型
#[derive(Clone, Debug)]
pub struct MySQL01Pool(pub DatabaseConnection);

/// 为了实现 `Deref` trait，我们需要手动实现 `MySQL01DatabaseConnection` 的 `Deref` trait
impl std::ops::Deref for MySQL01Pool {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// MySQL 数据库连接池初始化
pub async fn init_mysql_conn_pool(
    db_name: &str,
    param_map: &HashMap<String, String>,
) -> Result<DatabaseConnection, Error> {
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
        anyhow!("MySQL 数据库连接池({}) is {}", db_name, err)
    })?;

    Ok(pool)
}
