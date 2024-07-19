use std::collections::HashMap;

use lazy_static::lazy_static;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

use libproto::login_service_client::LoginServiceClient;
use libproto::LoginRequest;
use metaforge::generate_function_for_grpc_client;
use metaforge::grpc_server::{get_login_service_client, MyInterceptor};

lazy_static! {

    // 从配置文件中获取 gRPC 服务器地址
    static ref c01: HashMap<String, HashMap<String, String>> = metaforge::config::init_server_config().unwrap();
    static ref GRPC_ADDRESS: &'static str = Box::leak(Box::new(format!("http://{}:{}", c01.get("main").unwrap().get("mn_grpc_host").unwrap(), c01.get("main").unwrap().get("mn_grpc_port").unwrap())));

}

// 生成获取 gRPC 用户认证服务客户端函数
generate_function_for_grpc_client!(LoginServiceClient, GRPC_ADDRESS);

/// 测试用户登录
#[tokio::test]
async fn test_do_login() {

    let mut client = get_login_service_client(*GRPC_ADDRESS).await.unwrap();
    let request = tonic::Request::new(LoginRequest {
        usercode: "07788".to_string(),
        password: "123456".to_string(),
    });
    let resp = client.do_login(request).await.unwrap();
    let reply = resp.into_inner();
    dbg!(&reply);
    assert_eq!(reply.status, 0);

}

