use rand::Rng;
use tonic::{Status};
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::transport::Channel;
pub use admin_server::*;
pub use calculator_server::*;
pub use category_server::*;
pub use login_server::*;
pub use topic_server::*;
pub use jwt_server::*;
use libproto::login_service_client::LoginServiceClient;

mod admin_server;
mod calculator_server;
mod category_server;
mod login_server;
mod topic_server;
mod jwt_server;

/// 获取无需 JWT 认证的 grpc 登录服务客户端
pub async fn get_login_service_client(addr: &'static str) -> Result<LoginServiceClient<Channel>, anyhow::Error> {
    let client: LoginServiceClient<Channel> = LoginServiceClient::new(
        Channel::from_static(addr).connect().await?,
    );
    Ok(client)
}

/// 用于生成获取 gRPC 服务客户端函数的宏
#[macro_export]
macro_rules! generate_function_for_grpc_client {
    // 模式：接受函数名、参数列表和函数体
    ($client1:ident, $address:ident) => {

        macro_rules! snake_case_fn {
            () => {
                camel_to_snake(&stringify!($client1))
            };
        }

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

/// 生成指定长度随机字符串
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

/// 驼峰命名转蛇形命名
fn camel_to_snake(name: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in name.chars() {
        if c.is_uppercase() {
            if result.len() != 0 && !result.ends_with('_') {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            capitalize_next = false;
        } else if c == ' ' {
            if capitalize_next {
                result.push('_');
            }
            capitalize_next = false;
        } else {
            result.push(c);
            capitalize_next = c == '_';
        }
    }

    result
}
