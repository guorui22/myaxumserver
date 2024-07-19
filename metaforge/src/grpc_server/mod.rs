use rand::Rng;
use tonic::{Request, Status};
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
pub use admin_server::*;
pub use calculator_server::*;
pub use category_server::*;
pub use login_server::*;
pub use topic_server::*;
pub use jwt_server::*;

mod admin_server;
mod calculator_server;
mod category_server;
mod login_server;
mod topic_server;
mod jwt_server;

/// 用于生成获取 gRPC 服务客户端函数的宏
#[macro_export]
macro_rules! generate_function_for_grpc_client {
    // 模式：接受函数名、参数列表和函数体
    ($client1:ident, $address:ident) => {
        // 展开为一个函数定义
        async fn $client1(jwt: String) -> Result<$client1<InterceptedService<Channel, MyInterceptor>>, anyhow::Error> {
            Ok(<$client1<Channel>>::with_interceptor(
                Channel::from_static(*$address).connect().await?,
                MyInterceptor{
                    jwt,
                },
            ))
        }
    };
}

/// 自定义拦截器: 获取 JWT 认证 Token
pub struct MyInterceptor {
    pub jwt: String,
}

impl Interceptor for MyInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let token: MetadataValue<_> = self.jwt.parse().or_else(|e| Err(Status::internal(format!("invalid token {:?}", e))))?;
        request.metadata_mut().insert("authorization", token);
        Ok(request)
    }
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
