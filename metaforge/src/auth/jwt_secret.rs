use chrono::{Duration, Local, TimeDelta};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde_json::json;
use std::collections::HashMap;

use crate::auth::{AuthError, Claims};

/// 测试用数据: Token 过期时间，单位秒
pub const TOKEN_EXP: i64 = 1200;

lazy_static! {
    /// 测试用数据: 用户列表
    pub static ref USER_MAP: HashMap<String, serde_json::Value> = {
        let mut map = HashMap::new();
        map.insert("07733".to_string(), json!({
            "code": "07733",
            "name": "郭睿",
            "pwd": "123581321",
        }));
        map
    };
}

/// 测试方法：从 USER_MAP 中获取用户对象
/// id      用户唯一ID
/// pwd     用户密码
pub fn get_auth_user(code: &str, pwd: &str) -> Option<&'static serde_json::Value> {
    USER_MAP
        .get(code)
        .and_then(|user| if user["pwd"] == pwd { Some(user) } else { None })
}

/// JWT 密钥
/// secret   密钥
/// iss      签发者
#[derive(Debug, Clone)]
pub struct JwtSecret {
    pub secret: String,
    pub iss: String,
}

impl JwtSecret {
    /// 构造函数
    pub fn new(secret: String, iss: String) -> Self {
        Self { secret, iss }
    }
    /// 生成 Claims 结构体的实例
    /// usercode   用户唯一编号
    /// name     用户名
    /// life     过期时间长度，单位秒
    pub fn create_claims(&self, usercode: String, name: String, life: i64) -> Claims {
        Claims {
            code: usercode,
            name,
            iss: self.iss.clone(),
            exp: JwtSecret::convert_exp_to_timestamp(life),
        }
    }
    /// 从一个已存在的 Claims 生成新的 Claims 结构体的实例
    pub fn create_claims_with_expire(&self, claims: Claims, your_exp: i64) -> Claims {
        self.create_claims(claims.code.clone(), claims.name.clone(), your_exp)
    }

    /// 计算 Token 过期时间长度为时间戳
    pub(crate) fn convert_exp_to_timestamp(exp: i64) -> i64 {
        (Local::now() + Duration::try_seconds(exp).unwrap_or(TimeDelta::zero())).timestamp()
    }
    /// 获取密钥的字节数组
    fn secret_bytes(&self) -> &[u8] {
        (self.secret).as_bytes()
    }
    /// 为 Claims 实例生成 Token
    pub fn to_token(&self, claims: &Claims) -> Result<String, AuthError> {
        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(self.secret_bytes()),
        )
        .map_err(|err| AuthError::TokenCreation(err.to_string()))
    }
    /// 验证 Token 并返回 Claims 实例
    pub fn verify_and_get(&self, token: &str) -> Result<Claims, AuthError> {
        let mut v = Validation::new(jsonwebtoken::Algorithm::HS256);
        v.set_issuer(&[self.iss.clone()]);
        let token_data = decode(token, &DecodingKey::from_secret(self.secret_bytes()), &v)
            .map_err(|err| AuthError::InvalidToken(err.to_string()))?;
        Ok(token_data.claims)
    }

}
