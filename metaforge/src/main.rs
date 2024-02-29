use std::collections::HashMap;

use axum::{Extension, Router};
use axum::extract::DefaultBodyLimit;
use axum::handler::Handler;
use axum::http::{HeaderName, Method, StatusCode};
use axum::routing::{get, get_service, post};
use tower::limit::ConcurrencyLimitLayer;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use libconfig::init_server_config;
use libdatabase::{init_mysql_conn_pool, init_redis_conn_pool, TestMySqlDb01, GrMySQLPool, Pool, Redis01, RedisPool};
use libglobal_request_id::MyMakeRequestId;
use libgrpc::{Calculator, Login};
use libproto::calculator_service_server::CalculatorServiceServer;
use libproto::login_service_server::LoginServiceServer;
use libtracing::{get_my_format, info, Level, tracing_subscriber};
#[cfg(debug_assertions)]
use libtracing::get_my_stdout_writer;
use metaforge::handler::{get_jwt_token, get_protected_content, index, login_action, logout_action, mysql_query, mysql_transaction, redirect01, redirect02, upload_file, upload_file_action, UploadPath, user_login, user_main};
#[cfg(not(debug_assertions))]
use libtracing::get_my_file_writer;
#[cfg(not(debug_assertions))]
use libtracing::tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

#[tokio::main]
async fn main() -> Result<(), String> {

    // 如果监听到 ctrl+c 信号就退出应用
    ctrlc::set_handler(|| {
        info!("监听到 CTRL + C 操作, 退出应用程序.");
        std::process::exit(0);
    }).unwrap_or_else(|err| {
        panic!(
            "{}",
            err.to_string()
        )
    });

    // 读取服务器初始化参数
    let ini = init_server_config()?;
    let ini_main: &HashMap<String, String> = ini.get("MAIN").ok_or("MAIN section not found".to_string())?;

    // release模式下，日志输出到文件
    #[cfg(not(debug_assertions))]
        let ini_main_mn_log_path = ini_main
        .get("MN_LOG_PATH")
        .ok_or("MN_LOG_PATH not found".to_string())?;
    #[cfg(not(debug_assertions))]
        let ini_main_mn_log_name = ini_main
        .get("MN_LOG_NAME")
        .ok_or("MN_LOG_NAME not found".to_string())?;
    #[cfg(not(debug_assertions))]
        let (my_writer, _worker_guard): (NonBlocking, WorkerGuard) =
        get_my_file_writer(ini_main_mn_log_path, ini_main_mn_log_name);

    // debug模式下，日志输出到标准输出
    #[cfg(debug_assertions)]
        let my_writer: fn() -> std::io::Stdout = get_my_stdout_writer();

    // 初始化：设置日志等级、日志输出位置、日志格式(定制和筛选日志)
    let ini_log_level = ini_main.get("MN_LOG_LEVEL").map(|s| s.to_uppercase());
    let my_log_level = match ini_log_level {
        Some(s) => match s.as_str() {
            "TRACE" => Level::TRACE,
            "DEBUG" => Level::DEBUG,
            "INFO" => Level::INFO,
            "WARN" => Level::WARN,
            "ERROR" => Level::ERROR,
            _ => Level::INFO,
        },
        None => Level::INFO,
    };
    tracing_subscriber::fmt()
        .with_max_level(my_log_level)
        .with_writer(my_writer) // 写入标准输出，或者写入文件
        .event_format(get_my_format())
        .init();

    // 获取配置文件中的 MYSQL_01 配置信息
    let ini_mysql_01 = ini
        .get("MYSQL_01")
        .ok_or(format!("{} section not found", "MYSQL_01"))?;
    // 初始化 MYSQL_01 数据库连接池
    let test_mysql_db_01_pool: GrMySQLPool<TestMySqlDb01> = init_mysql_conn_pool::<TestMySqlDb01>(ini_mysql_01).await?;

    // 获取配置文件中的 REDIS_01 配置信息
    let ini_redis_01 = ini
        .get("REDIS_01")
        .ok_or("REDIS_01 section not found".to_string())?;
    // 初始化 REDIS_01 数据库连接池
    let redis_01_pool: Pool = init_redis_conn_pool("REDIS_01", ini_redis_01).await?;

    // 创建请求到服务之间的路由 router
    let x_request_id = HeaderName::from_static("x-request-id");

    // 启动应用监听本地 5000 端口
    let router = Router::new()
        .nest_service(
            "/static",
            get_service(ServeDir::new("static")).handle_error(|err| async move {
                (
                    StatusCode::NOT_FOUND,
                    format!("处理静态资源出错：{:?}", err),
                )
            }),
        )
        .route("/", get(index))
        // mysql数据库批量查询
        .route("/mysql_qry", post(mysql_query))
        // mysql数据库批量事务
        .route("/mysql_trans", post(mysql_transaction))
        // 使用用户名密码获取 jwt token
        .route("/get_jwt_token", get(get_jwt_token))
        // 使用有效的 jwt token 可访问受保护的内容
        .route("/get_protected_content", get(get_protected_content))
        // 本站页面跳转
        .route("/redirect01", get(redirect01))
        // 外站页面跳转
        .route("/redirect02", get(redirect02))
        // Session 用户登录页面 & 登录动作
        .route("/login", get(user_login).post(login_action))
        // Session 用户首页页面
        .route("/main", get(user_main))
        // Session 用户登出动作
        .route("/logout", get(logout_action))
        // 文件上传页面
        .route("/uploadfile", get(upload_file).post(upload_file_action.layer(ConcurrencyLimitLayer::new(5))))
        .layer(
            ServiceBuilder::new()
                // 禁用请求体大小默认2MB的限制
                .layer(DefaultBodyLimit::disable())
                // 限制请求体大小为 100MB
                .layer(RequestBodyLimitLayer::new(100 * 1024 * 1024))
                // 共享MySQL01数据库连接池
                .layer(Extension(test_mysql_db_01_pool))
                // 共享Redis01数据库连接池
                .layer(Extension(RedisPool::<Redis01>::new(redis_01_pool)))
                // 共享文件上传路径
                .layer(Extension(UploadPath { upload_path: ini_main.get("MN_UPLOAD_PATH").ok_or("获取文件上传路径出错。".to_string())?.to_string() }))
                // 启用数据压缩
                .layer(CompressionLayer::new())
                // 设置全局请求ID `x-request-id` 到所有的请求头中
                .layer(SetRequestIdLayer::new(
                    x_request_id.clone(),
                    MyMakeRequestId,
                ))
                // 传播全局请求ID `x-request-id` 从请求头到响应头中
                .layer(PropagateRequestIdLayer::new(x_request_id))
                // 启用http请求日志追踪
                .layer(TraceLayer::new_for_http())
                // 启用跨域请求控制
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST])
                        .allow_origin(Any),
                )
        );
    let host = ini_main.get("MN_SERVER_HOST").map_or("127.0.0.1", |h| h);
    let port = ini_main.get("MN_SERVER_PORT").map_or("5000", |p| p);


    // 启动 grpc 服务
    let addr = "0.0.0.0:29029";
    println!("grpc-srv run at: {}", addr);
    let calculater_srv = Calculator;
    let login_srv = Login;
    tonic::transport::Server::builder()
        .add_service(CalculatorServiceServer::with_interceptor(calculater_srv, libgrpc::check_auth))
        .add_service(LoginServiceServer::with_interceptor(login_srv, libgrpc::check_auth))
        .serve(addr.parse().unwrap())
        .await.map_err(|err| {
        format!("服务启动失败：{:?}", err)
    })?;

    // 启动 http 服务
    let listener = tokio::net::TcpListener::bind(&format!("{host}:{port}")).await.unwrap();
    axum::serve(listener, router).await.map_err(|err| {
        format!("服务启动失败：{:?}", err)
    })?;

    Ok(())
}