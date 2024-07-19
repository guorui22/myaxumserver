use tonic::codegen::InterceptedService;
use libproto::calculator_service_client::CalculatorServiceClient;
use libproto::Input;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::Request;
use tonic::Status;
use metaforge::get_grpc_client;

/// 服务器地址
const TEST_ADDRESS: &'static str = "http://172.17.0.1:29029";

/// JWT 认证 Token
const TEST_JWT: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjb2RlIjoiMDc3ODgiLCJuYW1lIjoiXCLpg63nnb9cIiIsImlzcyI6IuWco-WGnOmbhuWboiIsImV4cCI6MTcyMTQxNDc5MX0.FnsjW0ee12B_BNbTpCuoM6qUj_vB2Nffg09DEVw0NeM";

/// 访问无 JWT 认证 grpc 服务
#[tokio::test]
async fn test_find_square() {
    let mut client: CalculatorServiceClient<InterceptedService<Channel, fn(Request<()>) -> Result<Request<()>, Status>>> = get_grpc_client!(CalculatorServiceClient<Channel>, TEST_ADDRESS, TEST_JWT).unwrap();
    let request = tonic::Request::new(Input { number: 8 });
    let resp = client.find_square(request).await.unwrap();
    let reply = resp.into_inner();
    dbg!(&reply);
    assert!(reply.result > 0);
}
