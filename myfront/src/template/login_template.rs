use askama::Template;

/// 登录页模板
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub msg: String,
}
