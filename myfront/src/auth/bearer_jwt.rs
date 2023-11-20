use std::collections::HashMap;

use chrono::{Duration, Local};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde_json::json;

use crate::auth::{AuthError, Claims};

// 测试用数据
// 1、用户列表
// 2、JWT密钥
lazy_static! {
    pub static ref USER_MAP: HashMap<String, serde_json::Value> = {
        let mut map = HashMap::new();
        map.insert("07733".to_string(), json!({
            "id": "07733",
            "name": "郭睿",
            "pwd": "123581321",
        }));
        map
    };
    pub static ref JWT: Jwt = Jwt::new("不负信赖".to_string(), "圣农集团".to_string());
}


/// JWT 密钥
/// secret   密钥
/// iss      签发者
pub struct Jwt {
    pub secret: String,
    pub iss: String,
}

impl Jwt {
    /// 构造函数
    pub fn new(secret: String, iss: String) -> Self {
        Self { secret, iss }
    }
    /// 生成 Claims 结构体的实例
    /// id       用户唯一ID
    /// name     用户名
    /// life     过期时间长度，单位秒
    pub fn new_claims(&self, id: String, name: String, life: i64) -> Claims {
        Claims {
            id,
            name,
            iss: self.iss.clone(),
            exp: Jwt::calc_claims_exp(life),
        }
    }
    /// 从一个已存在的 Claims 生成新的 Claims 结构体的实例
    pub fn new_claims_with(&self, claims: Claims) -> Claims {
        self.new_claims(claims.id.clone(), claims.name.clone(), claims.exp)
    }

    /// 计算 Token 过期时间
    pub(crate) fn calc_claims_exp(exp: i64) -> i64 {
        (Local::now() + Duration::seconds(exp)).timestamp()
    }
    /// 获取密钥的字节数组
    fn secret_bytes(&self) -> &[u8] {
        (self.secret).as_bytes()
    }
    /// 为 Claims 实例生成 Token
    pub fn token(&self, claims: &Claims) -> Result<String, AuthError> {
        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(self.secret_bytes()),
        )
            .map_err(|err|AuthError::TokenCreation(err.to_string()))
    }
    /// 验证 Token 并返回 Claims 实例
    pub fn verify_and_get(&self, token: &str) -> Result<Claims, AuthError> {
        let mut v = Validation::new(jsonwebtoken::Algorithm::HS256);
        v.set_issuer(&[self.iss.clone()]);
        let token_data = decode(token, &DecodingKey::from_secret(self.secret_bytes()), &v)
            .map_err(|err|AuthError::InvalidToken(err.to_string()))?;
        Ok(token_data.claims)
    }
}




