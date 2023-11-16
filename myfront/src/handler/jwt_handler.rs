use axum::http::HeaderMap;
use axum::Json;
use axum_macros::debug_handler;
use serde_json::json;
use tracing::info;
use crate::jwt::{AuthInfo, authorize, Claims};
use crate::share::get_request_id;

/// 使用用户名&密码获取 JWT Token
#[debug_handler]
pub async fn get_jwt_token(Json(auth_info): Json<AuthInfo>) -> Result<Json<serde_json::Value>, String> {
    let auth_token = authorize(auth_info).map_err(|err|format!("用户Token验证失败: {}", err))?;
    Ok(Json(json!({
        "status": 0,
        "result": auth_token,
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
