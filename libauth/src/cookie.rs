use axum_extra::headers::HeaderMap;

/// Session ID 的 Cookie 名称，用在前端浏览器中
pub const TOKEN_NAME_FOR_COOKIE: &str = "axum_rs_token";
/// Session ID 的前缀，用在 Redis 数据库显示为文件夹
pub const SESSION_PREFIX_FOR_REDIS: &str = "axum_rs_session:";

/// 将 Session ID 保存到浏览器 Cookie
pub fn save_session_id_to_cookie(session_id: &str, headers: &mut HeaderMap) {
    let cookie = if session_id.is_empty() {
        // 设置 Cookie 过期
        format!("{}=;expires=Thu, 01 Jan 1970 00:00:01 GMT;path=/", TOKEN_NAME_FOR_COOKIE)
    } else {
        // 设置 Cookie 中的 Token 值
        format!("{}={};path=/", TOKEN_NAME_FOR_COOKIE, session_id)
    };
    headers.insert(
        axum::http::header::SET_COOKIE,
        cookie.parse().unwrap(),
    );
}

/// 从浏览器 Cookie 中获取 Session ID
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
        if cookie_name == TOKEN_NAME_FOR_COOKIE && !cookie_value.is_empty() {
            session_id = Some(cookie_value.to_string());
            break;
        }
    }
    session_id
}