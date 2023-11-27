use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// 自定错误信息
#[derive(thiserror::Error, Debug, Serialize)]
pub enum AuthError {
    #[error("生成 Token 时发生错误:{0}")]
    TokenCreation(String),
    #[error("Token 无效:{0}")]
    InvalidToken(String),
    #[error("用户不存在:{0}")]
    UserNotExist(String),
    #[error("用户信息不完整:{0}")]
    UserInfoIncomplete(String),
    #[error("认证信息错误:{0}")]
    WrongCredentials(String),
    #[error("认证信息缺失:{0}")]
    MissingCredentials(String),
}

/// implement the trait `axum::response::` for `auth::bearer_jwt::AuthError`
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        self.to_string().into_response()
    }
}
