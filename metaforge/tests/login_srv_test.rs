use std::collections::HashMap;
use anyhow::anyhow;
use lazy_static::lazy_static;
use libproto::login_service_client::LoginServiceClient;
use libproto::{Input, LoginReplyData, LoginRequest};
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::{Request, Status};
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use libproto::calculator_service_client::CalculatorServiceClient;
use metaforge::get_grpc_client;
use metaforge::model::global_const;

lazy_static! {

    static ref c01: HashMap<String, HashMap<String, String>> = metaforge::config::init_server_config().unwrap();
    // gRPC 服务器地址
    pub static ref GRPC_ADDRESS: &'static str = Box::leak(Box::new(format!("http://{}:{}", c01.get("main").unwrap().get("mn_grpc_host").unwrap(), c01.get("main").unwrap().get("mn_grpc_port").unwrap())));
}

#[tokio::test]
async fn test_do_login() {

    dbg!(*GRPC_ADDRESS);
    let mut client = get_client_no_inter(*GRPC_ADDRESS).await.unwrap();

    let request = tonic::Request::new(LoginRequest {
        usercode: "07788".to_string(),
        password: "123456".to_string(),
    });
    let resp = client.do_login(request).await.unwrap();
    let reply = resp.into_inner();
    dbg!(&reply);
    assert_eq!(reply.status, 0);

    if let Some(LoginReplyData { jwt: Some(mut jwt_value), .. }) = reply.data {
        jwt_value = format!("Bearer {}", jwt_value);
        let mut client01 = CalculatorServiceClient(jwt_value).await.unwrap();
        let request = tonic::Request::new(Input { number: 8 });
        let resp = client01.find_square(request).await.unwrap();
        let reply = resp.into_inner();
        dbg!(&reply);
        assert!(reply.result > 0);
    } else {
        println!("JWT not found");
    }

}

// type F117 = fn(Request<()>) -> Result<Request<()>, Status>;

// async fn get_client(addr:&'static str, jwt:&str) -> Result<LoginServiceClient<InterceptedService<Channel, F117>>, anyhow::Error> {
//     let client: LoginServiceClient<InterceptedService<Channel, F117>> = LoginServiceClient::with_interceptor(
//         Channel::from_static(addr).connect().await?,
//         |mut req: Request<()>|  -> Result<tonic::Request<()>, Status>{
//             let token: MetadataValue<_> = jwt.parse().or_else(|e| Err(Status::internal(format!("invalid token {:?}", e))))?;
//             req.metadata_mut().insert("authorization", token);
//             Ok(req)
//         },
//     );
//     Ok(client)
// }

/// 获取无需 JWT 认证的 grpc 服务客户端
async fn get_client_no_inter(addr: &'static str) -> Result<LoginServiceClient<Channel>, anyhow::Error> {
    let client: LoginServiceClient<Channel> = LoginServiceClient::new(
        Channel::from_static(addr).connect().await?,
    );
    Ok(client)
}


struct MyInterceptor {
    jwt: String,
}

impl Interceptor for MyInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let token: MetadataValue<_> = self.jwt.parse().or_else(|e| Err(Status::internal(format!("invalid token {:?}", e))))?;
        request.metadata_mut().insert("authorization", token);
        Ok(request)
    }
}

async fn get_client_gjw(addr:&'static str, jwt: String) -> Result<LoginServiceClient<InterceptedService<Channel, MyInterceptor>>, anyhow::Error> {
    let client: LoginServiceClient<InterceptedService<Channel, MyInterceptor>> = LoginServiceClient::with_interceptor(
        Channel::from_static(addr).connect().await?,
        MyInterceptor{
            jwt,
        }
    );
    Ok(client)
}

/// 宏定义：初始化 grpc 客户端
#[macro_export]
macro_rules! make_function_test {
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

make_function_test!(LoginServiceClient, GRPC_ADDRESS);
make_function_test!(CalculatorServiceClient, GRPC_ADDRESS);
