use anyhow::anyhow;
use lazy_static::lazy_static;
use libproto::login_service_client::LoginServiceClient;
use libproto::LoginRequest;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::{Request, Status};
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use metaforge::get_grpc_client;
use metaforge::model::global_const;

lazy_static! {
    pub static ref GRPC_ADDRESS: &'static str = "http://172.17.0.1:29029";
    pub static ref GRPC_JWT: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjbGllbnRfaWQiOiIwNzczMyIsImNsaWVudF9uYW1lIjoi6YOt552_IiwiZXhwIjo0ODUzMTg3MzU3fQ.febzZPRQmB8br5HVitBvZar4rf1WSf80CNtE0WtnIFQ";
}

/// 服务器地址
static TEST_ADDRESS: &'static str = "http://172.17.0.1:29029";
// const TEST_ADDRESS: &'static str = "http://grpc.sunnercn.com:29029";

/// JWT 认证 Token
const TEST_JWT: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjbGllbnRfaWQiOiIwNzczMyIsImNsaWVudF9uYW1lIjoi6YOt552_IiwiZXhwIjo0ODUzMTg3MzU3fQ.febzZPRQmB8br5HVitBvZar4rf1WSf80CNtE0WtnIFQ";

/// 访问无 JWT 认证 grpc 服务
#[tokio::test]
async fn test_do_login() {
    // let mut client = LoginServiceClient().await.unwrap();
    // let mut client = get_client_01().await.unwrap();
    // let mut client = get_client_00().await.unwrap();
    let mut client = get_client_no_inter(TEST_ADDRESS, TEST_JWT).await.unwrap();
    // let mut client = get_client(TEST_ADDRESS, TEST_JWT).await.unwrap();
    // let mut client = get_client_gjw(TEST_ADDRESS, TEST_JWT).await.unwrap();

    let request = tonic::Request::new(LoginRequest {
        usercode: "07788".to_string(),
        password: "123456".to_string(),
    });
    let resp = client.do_login(request).await.unwrap();
    let reply = resp.into_inner();
    dbg!(&reply);
    assert_eq!(reply.status, 0);
}

async fn get_client_00() -> Result<LoginServiceClient<InterceptedService<Channel, MyInterceptor>>, anyhow::Error> {
    get_grpc_client_test!(LoginServiceClient<Channel>, TEST_ADDRESS, TEST_JWT)
    // get_grpc_client!(LoginServiceClient<Channel>, TEST_ADDRESS, TEST_JWT)
}

type F117 = fn(Request<()>) -> Result<Request<()>, Status>;
async fn get_client(addr:&'static str, jwt:&str) -> Result<LoginServiceClient<InterceptedService<Channel, F117>>, anyhow::Error> {
    let client: LoginServiceClient<InterceptedService<Channel, F117>> = LoginServiceClient::with_interceptor(
        Channel::from_static(addr).connect().await?,
        |mut req: Request<()>|  -> Result<tonic::Request<()>, Status>{
            let token: MetadataValue<_> = TEST_JWT.parse().or_else(|e| Err(Status::internal(format!("invalid token {:?}", e))))?;
            req.metadata_mut().insert("authorization", token);
            Ok(req)
        },
    );
    Ok(client)
}

async fn get_client_no_inter(addr:&'static str, jwt:&str) -> Result<LoginServiceClient<Channel>, anyhow::Error> {
    let client: LoginServiceClient<Channel> = LoginServiceClient::new(
        Channel::from_static(addr).connect().await?,
    );
    Ok(client)
}


// You can also use the `Interceptor` trait to create an interceptor type
// that is easy to name
struct MyInterceptor {
    jwt: &'static str,
}

impl Interceptor for MyInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let token: MetadataValue<_> = self.jwt.parse().or_else(|e| Err(Status::internal(format!("invalid token {:?}", e))))?;
        request.metadata_mut().insert("authorization", token);
        Ok(request)
    }
}

async fn get_client_gjw(addr:&'static str, jwt:&'static str) -> Result<LoginServiceClient<InterceptedService<Channel, MyInterceptor>>, anyhow::Error> {
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
macro_rules! get_grpc_client_test {
    ($ty1:ident<$ty2:ty>, $address:ident, $token:ident) => {
        Ok(<$ty1<$ty2>>::with_interceptor(
            <$ty2>::from_static($address).connect().await?,
            MyInterceptor{
                jwt: $token,
            },
        ));
    };
}


#[macro_export]
macro_rules! make_function {
    // 模式：接受函数名、参数列表和函数体
    ($client1:ident) => {
        // 展开为一个函数定义
        async fn $client1() -> Result<$client1<InterceptedService<Channel, F117>>, anyhow::Error> {
            Ok(<$client1<Channel>>::with_interceptor(
                Channel::from_static(TEST_ADDRESS).connect().await?,
                |mut req: Request<()>| {
                    let token: MetadataValue<_> = TEST_JWT.parse().or_else(|e| Err(Status::internal(format!("invalid token {:?}", e))))?;
                    req.metadata_mut().insert("authorization", token);
                    Ok(req)
                },
            ))
        }
    };
}

#[macro_export]
macro_rules! make_function_test {
    // 模式：接受函数名、参数列表和函数体
    ($client1:ident, $address:ident, $token:ident) => {
        // 展开为一个函数定义
        async fn $client1() -> Result<$client1<InterceptedService<Channel, MyInterceptor>>, anyhow::Error> {
            Ok(<$client1<Channel>>::with_interceptor(
                Channel::from_static(&$address).connect().await?,
                MyInterceptor{
                    jwt: &$token,
                },
            ))
        }
    };
}


make_function_test!{LoginServiceClient, GRPC_ADDRESS, GRPC_JWT}
// make_function!{LoginServiceClient}
