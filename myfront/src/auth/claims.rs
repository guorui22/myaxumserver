use std::fmt::Display;

use axum::{async_trait, TypedHeader};
use axum::extract::FromRequestParts;
use axum::headers::{Authorization, Cookie};
use axum::headers::authorization::Bearer;
use axum::http::request::Parts;
use axum::http::StatusCode;
use chrono::{Local, LocalResult, TimeZone};
use serde::{Deserialize, Serialize};

use crate::auth::{Jwt, JWT, SESSION_ID_NAME_FOR_COOKIE};

/// 经过认证的用户信息
/// id      用户唯一ID
/// name    用户名
/// iss     签发者
/// exp     过期时间点，单位秒，从1970-01-01T00:00:00Z开始计算
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Claims {
    pub id: String,
    pub name: String,
    pub iss: String,
    pub exp: i64,
}

/// 实现 Display trait 可以保证此结构体可以直接打印输出
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let exp = Local.timestamp_opt(self.exp, 0).unwrap().to_string();
        write!(
            f,
            "Id: {}\nName: {}\nExpire: {:?}",
            self.id, self.name, exp)
    }
}

/// 实现 FromRequestParts trait 可以保证此结构体可以直接从请求中提取出来
#[async_trait]
impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Get the token from the request, either from the Authorization header or from the Cookie header
        let token = if let Ok(TypedHeader(Authorization(bearer))) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
            bearer.token().to_string()
        } else if let Ok(TypedHeader(cookies)) = TypedHeader::<Cookie>::from_request_parts(parts, state).await {
            cookies.get(SESSION_ID_NAME_FOR_COOKIE).map(|cookie| cookie.to_string()).unwrap_or_default()
        } else {
            return Err((StatusCode::BAD_REQUEST, "Missing Authorization Header".to_string()));
        };

        // Decode the user data
        let claims = JWT.verify_and_get(&token).map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

        // Check the token expiration
        let exp = if let LocalResult::Single(expire_datetime) = Local.timestamp_opt(Jwt::calc_claims_exp(claims.exp), 0) {
            expire_datetime
        } else {
            return Err((StatusCode::BAD_REQUEST, "Invalid Claims Expire".to_string()));
        };
        if exp < Local::now() {
            return Err((StatusCode::BAD_REQUEST, "Token Expired".to_string()));
        }

        Ok(claims)
    }
}
