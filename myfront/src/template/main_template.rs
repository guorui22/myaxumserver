use askama::Template;

/// 用户首页模板
#[derive(Template)]
#[template(path = "main.html")]
pub struct MainTemplate {
    pub username: String,
    pub level: u8,
}