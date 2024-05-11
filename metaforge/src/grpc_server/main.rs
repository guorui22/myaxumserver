use libgrpc::{Admin, Calculator, Category, Login, Topic};
use libproto::admin_service_server::AdminServiceServer;
use libproto::calculator_service_server::CalculatorServiceServer;
use libproto::category_service_server::CategoryServiceServer;
use libproto::login_service_server::LoginServiceServer;
use libproto::topic_service_server::TopicServiceServer;
use sqlx::MySqlPool;
use std::env;
use std::sync::Arc;
use libauth::Jwt;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:29029";
    println!("grpc-srv run at: {}", addr);

    let jwt_exp: i64 = 3600;
    let jwt = Jwt::new("不负信赖".to_string(), "圣农集团".to_string());

    let calculater_srv = Calculator;
    let login_srv = Login {
        jwt,
        jwt_exp,
    };

    tonic::transport::Server::builder()
        // .add_service(AdminServiceServer::with_interceptor(admin_srv, libgrpc::check_auth))
        // .add_service(CategoryServiceServer::with_interceptor(category_srv,libgrpc::check_auth))
        // .add_service(TopicServiceServer::with_interceptor(topic_srv, libgrpc::check_auth))
        .add_service(CalculatorServiceServer::with_interceptor(
            calculater_srv,
            libgrpc::check_auth,
        ))
        .add_service(LoginServiceServer::with_interceptor(
            login_srv,
            libgrpc::check_auth,
        ))
        // .add_service(CalculatorServiceServer::new(calculater_srv))
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}
