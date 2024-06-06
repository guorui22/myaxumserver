use axum::extract::Request;
use axum::http::{HeaderMap, HeaderValue};
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

/// 全局唯一请求ID结构体
#[derive(Clone)]
pub struct MyMakeRequestId;

/// 实现 `MakeRequestId` trait，为每个请求生成一个唯一的请求ID
impl MakeRequestId for MyMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();
        HeaderValue::from_str(request_id.as_ref()).map_or(None, |x| Some(RequestId::new(x)))
    }
}

/// 获取请求头中的 x-request-id
pub fn get_request_id(headers: &HeaderMap) -> String {
    headers.get("x-request-id").map_or_else(
        || "".to_string(),
        |x| x.to_str().map_or_else(|_| "".to_string(), |x| x.to_string()),
    )
}
