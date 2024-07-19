use rand::Rng;
use tonic::{Request, Status};
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

/// 宏定义：初始化 grpc 客户端
#[macro_export]
macro_rules! get_grpc_client {
    ($ty1:ident<$ty2:ty>, $address:ident, $token:ident) => {
        Ok(<$ty1<$ty2>>::with_interceptor(
            <$ty2>::from_static($address).connect().await?,
            |mut req: Request<()>| -> Result<tonic::Request<()>, Status> {
                let token: MetadataValue<_> = $token.parse().or_else(|e| Err(Status::internal(format!("invalid token {:?}", e))))?;
                req.metadata_mut().insert("authorization", token);
                Ok(req)
            },
        ));
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
