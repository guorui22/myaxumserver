use axum::http::HeaderMap;

/// 获取请求头中的 x-request-id
pub fn get_request_id(headers: &HeaderMap) -> String {
    headers
        .get("x-request-id")
        .map_or_else(||"None".to_string(), |x| x.to_str().map_or_else(|e| e.to_string(), |x| x.to_string()))
}