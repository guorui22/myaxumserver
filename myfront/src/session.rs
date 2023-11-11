use axum::headers::HeaderMap;
use serde::{Deserialize, Serialize};

/// Session ID 的 Cookie 名称，用在前端浏览器中
const SESSION_ID_COOKIE_NAME: &str = "axum_rs_session_id";
/// Session ID 的前缀，用在 Redis 数据库显示为文件夹
pub const SESSION_KEY_PREFIX: &str = "axum_rs_session:";

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

/// 将 Session ID 保存到 Cookie
pub fn save_session_id_to_cookie(session_id: &str, headers: &mut HeaderMap) {
    let cookie = if session_id.is_empty() {
        // 设置 Cookie 过期
        format!("{}=;expires=Thu, 01 Jan 1970 00:00:01 GMT;", SESSION_ID_COOKIE_NAME)
    } else {
        format!("{}={}", SESSION_ID_COOKIE_NAME, session_id)
    };
    headers.insert(
        axum::http::header::SET_COOKIE,
        cookie.as_str().parse().unwrap(),
    );
}

/// 从 cookie 中获取session id
pub fn get_session_from_cookie(headers: &HeaderMap) -> Option<String> {
    let cookies = headers
        .get(axum::http::header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    if cookies.is_empty() {
        return None;
    }
    let mut session_id: Option<String> = None;
    let cookies: Vec<&str> = cookies.split(';').collect();
    for cookie in cookies {
        let cookie_pair: Vec<&str> = cookie.split('=').collect();
        let cookie_name = cookie_pair[0].trim();
        let cookie_value = cookie_pair[1].trim();
        if cookie_name == SESSION_ID_COOKIE_NAME && !cookie_value.is_empty() {
            session_id = Some(cookie_value.to_string());
            break;
        }
    }
    session_id
}