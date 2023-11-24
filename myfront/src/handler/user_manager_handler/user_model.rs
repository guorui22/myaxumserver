use serde::{Deserialize, Serialize};

/// 用户登录表单
#[derive(Deserialize)]
pub struct UserLoginForm {
    pub username: String,
    pub password: String,
}

/// 用户登录界面的提示信息
#[derive(Deserialize)]
pub struct LoginMessage {
    pub msg: Option<String>,
}

/// 用户Session
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub username: String,
    pub level: u8,
}
