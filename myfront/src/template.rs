use askama::Template;

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
    pub level: u8,
}