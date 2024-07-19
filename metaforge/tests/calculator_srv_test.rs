use std::collections::HashMap;

use lazy_static::lazy_static;
use tonic::codegen::InterceptedService;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::Streaming;
use tonic::transport::Channel;

use libproto::{Input, LoginReplyData, LoginRequest, Output};
use libproto::calculator_service_client::CalculatorServiceClient;
use metaforge::generate_function_for_grpc_client;
use metaforge::grpc_server::{get_login_service_client, MyInterceptor};

lazy_static! {

    // 从配置文件中获取 gRPC 服务器地址
    static ref c01: HashMap<String, HashMap<String, String>> = metaforge::config::init_server_config().unwrap();
    static ref GRPC_ADDRESS: &'static str = Box::leak(Box::new(format!("http://{}:{}", c01.get("main").unwrap().get("mn_grpc_host").unwrap(), c01.get("main").unwrap().get("mn_grpc_port").unwrap())));

}

// 生成获取 gRPC 计算服务客户端函数
generate_function_for_grpc_client!(CalculatorServiceClient, GRPC_ADDRESS);

/// 测试计算服务
#[tokio::test]
async fn test_find_square() {

    let mut client = get_login_service_client(*GRPC_ADDRESS).await.unwrap();
    let request = tonic::Request::new(LoginRequest {
        usercode: "07788".to_string(),
        password: "123456".to_string(),
    });
    let resp = client.do_login(request).await.unwrap();
    let reply = resp.into_inner();

    if let Some(LoginReplyData { jwt: Some(mut jwt_value), .. }) = reply.data {
        jwt_value = format!("Bearer {}", jwt_value);
        let mut client01 = CalculatorServiceClient(jwt_value).await.unwrap();

        // 测试平方计算服务
        let request = tonic::Request::new(Input { number: 8 });
        let resp = client01.find_square(request).await.unwrap();
        let reply = resp.into_inner();
        dbg!(&reply);
        assert!(reply.result > 0);

        // 测试因数计算服务
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
