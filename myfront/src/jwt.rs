use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Add;

use anyhow::anyhow;
use axum::{async_trait, TypedHeader};
use axum::extract::{FromRequest, FromRequestParts};
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::http::{Request, StatusCode};
use axum::http::request::Parts;
use chrono::{Duration, Local, LocalResult, TimeZone};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// 密钥结构体，其中包含由主密钥生成的加密密钥 encoding 和解密密钥 decoding
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

// 测试用数据
// 1、用户列表
// 2、JWT密钥
lazy_static! {
    static ref USER_MAP: HashMap<String, serde_json::Value> = {
        let mut map = HashMap::new();
        map.insert("07733".to_string(), json!({
            "client_id": "07733",
            "client_name": "郭睿",
            "client_pwd": "123581321",
        }));
        map
    };
    static ref JWT_KEYS: Keys = {
        let secret = "secret".to_string();
        Keys::new(secret.as_bytes())
    };
}

/// 客户端为获取授权 token 而向服务器提交信息的结构体
/// client_id       客户唯一ID
/// client_pwd   客户密码
#[derive(Debug, Deserialize)]
pub struct AuthInfo {
    client_id: String,
    client_pwd: String,
}

/// 返回给客户端包含有 token 的结构体
/// access_token    token 字符串
/// token_type      token 类型，默认是 Bearer
#[derive(Debug, Serialize)]
pub struct AuthToken {
    access_token: String,
    token_type: String,
}

/// 生成 AuthToken 结构体的实例
impl AuthToken {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

/// Claims 认证客户信息提取器
/// client_id      客户唯一ID
/// client_name   客户名称
/// expire_datetime 过期日期
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub client_id: String,
    pub client_name: String,
    pub exp: i64,
}

/// 实现 Display trait 可以保证此结构体可以直接打印输出
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let exp = Local.timestamp_opt(self.exp, 0).single().ok_or(std::fmt::Error)?;
        write!(
            f,
            "ClientId: {}\nClientPassword: {}\nExpireDateTime: {}",
            self.client_id, self.client_name, exp
        )
    }
}

/// 实现 FromRequest trait 可以保证此结构体可以直接从请求中提取出来
#[async_trait]
impl<S, B> FromRequest<S, B> for Claims
    where
        B: Send + 'static,
        S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req, state)
                .await
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid token".to_string()))?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &JWT_KEYS.decoding, &Validation::default())
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid Claims".to_string()))?;

        Ok(token_data.claims)
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
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid token".to_string()))?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &JWT_KEYS.decoding, &Validation::default())
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

        // Check the token expiration
        let exp = if let LocalResult::Single(expire_datetime) = Local.timestamp_opt(token_data.claims.exp, 0) {
            expire_datetime
        } else {
            return Err((StatusCode::BAD_REQUEST, "Invalid Claims Expire".to_string()));
        };
        if exp < Local::now() {
            return Err((StatusCode::BAD_REQUEST, "Token Expired".to_string()));
        }

        Ok(token_data.claims)
    }
}

/// 使用用户名密码生成 JWT Token 字符串，默认有效期100年
pub fn authorize(AuthInfo { client_id, client_pwd }: AuthInfo) -> Result<AuthToken, AuthError> {
    // 检验用户提交的信息是否有效
    if client_id.is_empty() || client_pwd.is_empty() {
        return Err(AuthError::MissingCredentials(anyhow!("用户名或密码为空。")));
    }
    // 检验用户编码是否存在
    let user = USER_MAP.get(&client_id).ok_or(
        AuthError::UserNotExist(anyhow!("用户不存在。")),
    )?;

    // 检验用户名和密码是否正确
    let id = user.get("client_id")
        .map_or(Err(AuthError::MissingCredentials(anyhow!("用户编号不存在。"))), |x| Ok(x.as_str()))?
        .ok_or(AuthError::MissingCredentials(anyhow!("用户编号不存在。")))?;
    let pwd = user.get("client_pwd")
        .map_or(Err(AuthError::MissingCredentials(anyhow!("用户密码不存在。"))), |x| Ok(x.as_str()))?
        .ok_or(AuthError::MissingCredentials(anyhow!("用户密码不存在。")))?;

    if !&id.eq(&client_id) || !&pwd.eq(&client_pwd) {
        return Err(AuthError::WrongCredentials(anyhow!("用户名或密码错误。")));
    }
    // 构造 token 重要包含的信息(token 过期时间很重要)
    let client_name = user.get("client_name")
        .map_or(Err(AuthError::UserInfoIncomplete(anyhow!("用户名称不存在。"))), |x| Ok(x.as_str()))?
        .ok_or(AuthError::UserInfoIncomplete(anyhow!("用户名称不存在。")))?
        .to_string();
    let claims = Claims {
        client_id,
        client_name,
        // exp: Local::now().add(Duration::seconds(30)).timestamp(),
        exp: Local::now().add(Duration::days(365 * 100)).timestamp(),
    };

    // Create the authorization token
    let token = encode(&Header::default(), &claims, &JWT_KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation(anyhow!("生成 Token 发生错误。")))?;

    // Send the authorized token
    Ok(AuthToken::new(token))
}

/// 自定错误信息
#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("用户不存在:{0}")]
    UserNotExist(anyhow::Error),
    #[error("用户信息不完整:{0}")]
    UserInfoIncomplete(anyhow::Error),
    #[error("认证信息错误:{0}")]
    WrongCredentials(anyhow::Error),
    #[error("认证信息缺失:{0}")]
    MissingCredentials(anyhow::Error),
    #[error("生成 Token 时发生错误:{0}")]
    TokenCreation(anyhow::Error),
    #[error("Token 无效:{0}")]
    InvalidToken(anyhow::Error),
}
