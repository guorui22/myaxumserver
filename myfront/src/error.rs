use std::fmt::Debug;

use tracing::error;

/// 自定义错误类型
#[derive(thiserror::Error, Debug)]
pub enum FrontError {
    #[error("未知错误:{0:?}")]
    UnknownError(String),
}