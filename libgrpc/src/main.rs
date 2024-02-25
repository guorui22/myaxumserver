use std::env;
use std::sync::Arc;
use sqlx::MySqlPool;
use libgrpc::{Admin, Calculator, Category, Login, Topic};
use libproto::admin_service_server::AdminServiceServer;
use libproto::calculator_service_server::CalculatorServiceServer;
use libproto::category_service_server::CategoryServiceServer;
use libproto::login_service_server::LoginServiceServer;
use libproto::topic_service_server::TopicServiceServer;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:29029";
    println!("grpc-srv run at: {}", addr);

    // let dsn = env::var("MYSQL_DSN").unwrap_or("mysql://root:6@Q29sX+38yz4Rep*^@172.17.0.1:4000/myblog".to_string());
    // let pool = MySqlPool::connect(&dsn).await.unwrap();
    // let arc_pool = Arc::new(pool);

    // let admin_srv = Admin::new(arc_pool.clone());
    // let category_srv = Category::new(arc_pool.clone());
    // let topic_srv = Topic::new(arc_pool);
    let calculater_srv = Calculator;
    let login_srv = Login;

    tonic::transport::Server::builder()
        // .add_service(AdminServiceServer::with_interceptor(admin_srv, libgrpc::check_auth))
        // .add_service(CategoryServiceServer::with_interceptor(category_srv,libgrpc::check_auth))
        // .add_service(TopicServiceServer::with_interceptor(topic_srv, libgrpc::check_auth))
        .add_service(CalculatorServiceServer::with_interceptor(calculater_srv, libgrpc::check_auth))
        .add_service(LoginServiceServer::with_interceptor(login_srv, libgrpc::check_auth))
        // .add_service(CalculatorServiceServer::new(calculater_srv))
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}