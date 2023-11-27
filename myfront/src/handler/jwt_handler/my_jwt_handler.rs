use axum::http::HeaderMap;
use axum::Json;
use axum_macros::debug_handler;
use serde_json::json;
use tracing::info;

use crate::auth::{AuthError, Claims, get_auth_user, JWT, TOKEN_EXP};
use crate::global_request_id::get_request_id;
use crate::handler::AuthInfo;

/// 使用用户名&密码获取 JWT Token
#[debug_handler]
pub async fn get_jwt_token(Json(AuthInfo{ client_code, client_pwd }): Json<AuthInfo>) -> Result<Json<serde_json::Value>, AuthError> {

    let user = match get_auth_user(&client_code, &client_pwd) {
        Some(user) => user,
        None => return Err(AuthError::WrongCredentials("用户ID或密码错误。".to_string())),
    };

    // 获取用户名
    let client_name = user.get("name").map_or("",|val| val.as_str().unwrap_or_default());

    // 构造 token 重要包含的信息(token 过期时间很重要)
    let claims = JWT.new_claims(client_code, client_name.to_string(), TOKEN_EXP);
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

