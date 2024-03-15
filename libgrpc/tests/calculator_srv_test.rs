use libgrpc::get_grpc_client;
use libproto::calculator_service_client::CalculatorServiceClient;
use libproto::Input;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::Request;

/// 服务器地址
const TEST_ADDRESS: &'static str = "http://172.17.0.1:29029";
// const TEST_ADDRESS: &'static str = "http://grpc.sunnercn.com:29029";

/// JWT 认证 Token
const TEST_JWT: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjbGllbnRfaWQiOiIwNzczMyIsImNsaWVudF9uYW1lIjoi6YOt552_IiwiZXhwIjo0ODUzMTg3MzU3fQ.febzZPRQmB8br5HVitBvZar4rf1WSf80CNtE0WtnIFQ";

/// 访问无 JWT 认证 grpc 服务
#[tokio::test]
async fn test_find_square() {
    let mut client = get_grpc_client!(CalculatorServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    // let mut client = CalculatorServiceClient::connect(TEST_ADDRESS).await.unwrap();
    let request = tonic::Request::new(Input { number: 9 });
    let resp = client.find_square(request).await.unwrap();
    let reply = resp.into_inner();
    dbg!(&reply);
    assert!(reply.result > 0);
}
