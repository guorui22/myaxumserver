use std::collections::HashMap;
use anyhow::anyhow;
use lazy_static::lazy_static;
use libproto::login_service_client::LoginServiceClient;
use libproto::{Input, LoginReplyData, LoginRequest, Output};
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::{Request, Status, Streaming};
use tonic::codegen::InterceptedService;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::service::Interceptor;
use libproto::calculator_service_client::CalculatorServiceClient;
use metaforge::{generate_function_for_grpc_client,};
use metaforge::model::global_const;
use metaforge::grpc_server::MyInterceptor;

lazy_static! {

    // 从配置文件中获取 gRPC 服务器地址
    static ref c01: HashMap<String, HashMap<String, String>> = metaforge::config::init_server_config().unwrap();
    pub static ref GRPC_ADDRESS: &'static str = Box::leak(Box::new(format!("http://{}:{}", c01.get("main").unwrap().get("mn_grpc_host").unwrap(), c01.get("main").unwrap().get("mn_grpc_port").unwrap())));
}

#[tokio::test]
async fn test_do_login() {

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

        let request = tonic::Request::new(Input { number: 81 });
        let mut response: Streaming<Output> = client01.find_factors(request).await.unwrap().into_inner();
        // 遍历 response 中的结果
        while let Some(resp) = response.next().await {
            let reply = resp.unwrap();
            dbg!(&reply);
            assert!(reply.result > 0);
        }

    } else {
        println!("JWT not found");
    }

}

/// 获取无需 JWT 认证的 grpc 服务客户端
async fn get_client_no_inter(addr: &'static str) -> Result<LoginServiceClient<Channel>, anyhow::Error> {
    let client: LoginServiceClient<Channel> = LoginServiceClient::new(
        Channel::from_static(addr).connect().await?,
    );
    Ok(client)
}



generate_function_for_grpc_client!(LoginServiceClient, GRPC_ADDRESS);
generate_function_for_grpc_client!(CalculatorServiceClient, GRPC_ADDRESS);
