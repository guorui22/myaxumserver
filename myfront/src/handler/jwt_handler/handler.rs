use axum::http::HeaderMap;
use axum::Json;
use axum_macros::debug_handler;
use serde_json::json;
use tracing::info;
use crate::auth;
use crate::auth::{AuthError, Claims, JWT};
use crate::handler::AuthInfo;
use crate::share::get_request_id;

/// 使用用户名&密码获取 JWT Token
#[debug_handler]
pub async fn get_jwt_token(Json(AuthInfo{ client_id: _client_id, client_pwd: _client_pwd }): Json<AuthInfo>) -> Result<Json<serde_json::Value>, AuthError> {

    // 检验用户提交的信息是否有效
    if _client_id.is_empty() || _client_pwd.is_empty() {
        return Err(AuthError::MissingCredentials("用户ID或密码为空。".to_string()));
    }
    // 检验用户ID是否存在
    let user = auth::USER_MAP.get(&_client_id).ok_or(
        AuthError::UserNotExist("用户不存在。".to_string())
    )?;

    // 检验用户ID和密码是否正确
    let id = user.get("id")
        .ok_or(AuthError::MissingCredentials("用户ID不存在。".to_string()))?;
    let pwd = user.get("pwd")
        .ok_or(AuthError::MissingCredentials("用户密码不存在。".to_string()))?;

    if !matches!(id, _client_id) && !matches!(pwd, _client_pwd) {
        return Err(AuthError::WrongCredentials("用户ID或密码错误。".to_string()));
    }

    // 获取用户名
    let client_name = user.get("name")
        .ok_or(AuthError::UserInfoIncomplete("用户名不存在。".to_string()))?;

    // 构造 token 重要包含的信息(token 过期时间很重要)
    let claims = JWT.new_claims(_client_id, client_name.to_string(), 30);
    let token = JWT.token(&claims).map_err(|err|AuthError::TokenCreation(err.to_string()))?;

    Ok(Json(json!({
        "status": 0,
        "result": token,
    })))
}

/// 使用 JWT Token 访问受保护的内容
#[debug_handler]
pub async fn get_protected_content(
    claims: Claims,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, String> {
    let request_id = get_request_id(&headers);
    info!("x-request-id={}", request_id);
    info!("claims={:?}", claims);

    Ok(Json(json!({
        "status": 0,
        "result": "Welcome to protected chronology!",
    })))
}

