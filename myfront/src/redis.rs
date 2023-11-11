use std::collections::HashMap;

use anyhow::{anyhow, Error};
use deadpool_redis::{Config, Connection, Pool, Runtime};
use tracing::error;

use crate::error::SfAllError;

/// 自定义数据库连接池类型
#[derive(Clone)]
pub struct Redis01Pool(pub Pool);

/// 为了实现 `Deref` trait，我们需要手动实现 `Redis01Pool` 的 `Deref` trait
impl std::ops::Deref for Redis01Pool {
    type Target = Pool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Redis 数据库连接池初始化
pub async fn init_redis_conn_pool(
    db_name: &str,
    param_map: &HashMap<String, String>,
) -> Result<Pool, Error> {
    let host_default = &String::from("127.0.0.1");
    let host = param_map.get("RD_HOST").unwrap_or(host_default);
    let port_default = &String::from("6379");
    let port = param_map.get("RD_PORT").unwrap_or(port_default);
    let pool = Config::from_url(format!("redis://{host}:{port}"))
        .create_pool(Some(Runtime::Tokio1))
        .map_or_else(
            |err| Err(SfAllError::DatabasePoolInitError(anyhow!(format!("Redis 数据库连接池({}) is {}", db_name, err)))),
            Ok)?;
    Ok(pool)
}

/// 从Redis数据库连接池获取连接
pub async fn get_redis_connection(pool: Pool) -> Result<Connection, String> {
    let conn = pool.get().await.map_err(|err| {
        let str_err = format!("Redis 获取连接失败：{}", err);
        error!("{str_err}");
        str_err
    })?;
    Ok(conn)
}