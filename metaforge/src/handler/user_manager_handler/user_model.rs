use askama::Template;
use serde::{Deserialize, Serialize};

/// 用户登录表单
#[derive(Deserialize)]
pub struct UserLoginForm {
    pub usercode: String,
    pub password: String,
}

/// 用户登录界面的提示信息
#[derive(Deserialize)]
pub struct LoginMessage {
    pub msg: Option<String>,
}

/// 用户 Session
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub username: String,
    pub level: u8,
}

/// 登录页模板
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub msg: String,
}

/// 用户首页模板
#[derive(Template)]
#[template(path = "main.html")]
pub struct MainTemplate {
    pub username: String,
    pub usercode: String,
    pub level: u8,
}