use libproto::Input;
use libproto::calculator_service_client::CalculatorServiceClient;

/// 服务器地址
const TEST_ADDRESS: &'static str = "http://172.17.0.1:29029";
// const TEST_ADDRESS: &'static str = "http://grpc.sunnercn.com:29029";

/// 访问无 JWT 认证 grpc 服务
#[tokio::test]
async fn test_find_square() {
    let mut client = CalculatorServiceClient::connect(TEST_ADDRESS).await.unwrap();
    let request = tonic::Request::new(Input {
        number: 9,
    });
    let resp = client.find_square(request).await.unwrap();
    let reply = resp.into_inner();
    dbg!(&reply);
    assert!(reply.result > 0);
}