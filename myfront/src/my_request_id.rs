use axum::http::{HeaderValue, Request};
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

/// 全局唯一请求ID结构体
#[derive(Clone)]
pub struct MyMakeRequestId;

/// 实现 `MakeRequestId` trait，为每个请求生成一个唯一的请求ID
impl MakeRequestId for MyMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();
        if let Ok(x_request_id) = HeaderValue::from_str(request_id.as_ref()) {
            Some(RequestId::new(x_request_id))
        } else {
            None
        }
    }
}
