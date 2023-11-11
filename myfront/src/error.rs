use axum::Json;
use serde_json::{json, Value};
use tracing::error;

/// 自定义 HTTP 错误处理(先输出到日志，再返回一个包含错误的字符串 JSON 对象)
pub fn axum_http_handle_err_process<T: ToString>(err: T) -> Json<Value> {
    let err_string = err.to_string();
    error!("{}", err_string);
    Json(json!({
        "status": -1,
        "result": err_string,
    }))
}

/// 自定义错误类型
#[derive(thiserror::Error, Debug)]
pub enum SfAllError {
    #[error("数据库连接池初始化失败:{0}")]
    DatabasePoolInitError(anyhow::Error),
    #[error("ctrl+c 退出应用失败:{0}")]
    CtrlcError(anyhow::Error),
    #[error("初始化应用配置参数失败:{0:?}")]
    InitConfError(anyhow::Error),
    #[error("初始化应用日志目录失败:{0:?}")]
    InitLogDirError(anyhow::Error),
    #[error("初始化 Redis Url 字符串失败:{0:?}")]
    InitRedisUrlError(anyhow::Error),
    #[error("初始化服务器 SocketAddr 失败:{0:?}")]
    InitSocketAddrError(anyhow::Error),
    #[error("未知的服务器内部错误:{0:?}")]
    UnknownError(anyhow::Error),
}
