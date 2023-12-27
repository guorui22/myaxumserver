mod admin_server;
mod category_server;
mod topic_server;
mod calculator_server;

use rand::Rng;
use tonic::metadata::MetadataValue;
use tonic::{Request, Status};
pub use admin_server::*;
pub use category_server::*;
pub use topic_server::*;
pub use calculator_server::*;

/// 检查请求头中的 token 是否正确
pub fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjbGllbnRfaWQiOiIwNzczMyIsImNsaWVudF9uYW1lIjoi6YOt552_IiwiZXhwIjo0ODUzMTg3MzU3fQ.febzZPRQmB8br5HVitBvZar4rf1WSf80CNtE0WtnIFQ".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

/// 宏定义：初始化 grpc 客户端
#[macro_export]
macro_rules! get_grpc_client {
    ($client: ty, $address: ident, $token: ident) => {
        <$client>::with_interceptor(Channel::from_static($address).connect().await.unwrap(), |mut req: Request<()>|{
            let token: MetadataValue<_> = $token.parse().unwrap();
            req.metadata_mut().insert("authorization", token);
            Ok(req)
        })
    };
}

pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    let random_string: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..charset.len());
            charset[index] as char
        })
        .collect();

    random_string
}
