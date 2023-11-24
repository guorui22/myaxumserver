use axum::http::HeaderMap;

/// 获取请求头中的 x-request-id
pub fn get_request_id(headers: &HeaderMap) -> String {
    headers
        .get("x-request-id")
        .map_or("".to_string(), |x| x.to_str().unwrap().to_string())
}