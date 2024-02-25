use tonic::transport::Channel;
use tonic::metadata::MetadataValue;
use tonic::Request;
use libgrpc::get_grpc_client;
use libproto::{LoginRequest};
use libproto::login_service_client::LoginServiceClient;

/// 服务器地址
const TEST_ADDRESS: &'static str = "http://172.17.0.1:29029";
// const TEST_ADDRESS: &'static str = "http://grpc.sunnercn.com:29029";

/// JWT 认证 Token
const TEST_JWT: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjbGllbnRfaWQiOiIwNzczMyIsImNsaWVudF9uYW1lIjoi6YOt552_IiwiZXhwIjo0ODUzMTg3MzU3fQ.febzZPRQmB8br5HVitBvZar4rf1WSf80CNtE0WtnIFQ";


/// 访问无 JWT 认证 grpc 服务
#[tokio::test]
async fn test_do_login() {
    let mut client = get_grpc_client!(LoginServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(LoginRequest {
        usercode: "admin".to_string(),
        password: "admin".to_string(),
    });
    let resp = client.do_login(request).await.unwrap();
    let reply = resp.into_inner();
    dbg!(&reply);
    assert!(reply.username.unwrap().len() > 0);
}