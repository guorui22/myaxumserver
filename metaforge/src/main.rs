use std::collections::HashMap;

use anyhow::anyhow;
use axum::{Extension, Router};
use axum::extract::DefaultBodyLimit;
use axum::handler::Handler;
use axum::http::{HeaderName, Method, StatusCode};
use axum::routing::{get, get_service, post};
use tokio::task::JoinHandle;
use tower::limit::ConcurrencyLimitLayer;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

use libdatabase::{
    GrMySQLPool, init_mysql_conn_pool, init_redis_conn_pool, Pool, Redis01, RedisPool,
    TestMySqlDb01,
};
use libproto::calculator_service_server::CalculatorServiceServer;
use libproto::jwt_service_server::JwtServiceServer;
use libproto::login_service_server::LoginServiceServer;
#[allow(unused_imports)]
use metaforge::my_tracing::{ get_my_tracing_format};
#[cfg(not(debug_assertions))]
use metaforge::my_tracing::get_my_tracing_file_writer;
#[cfg(debug_assertions)]
use metaforge::my_tracing::get_my_tracing_stdout_writer;
#[cfg(not(debug_assertions))]
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use metaforge::auth::grpc_check_auth;
use metaforge::grpc_server::{Calculator, Jwt, Login};
use metaforge::handler::{
    get_jwt_token, get_protected_content, index, login_action, logout_action, mysql_query,
    mysql_transaction, redirect01, redirect02, upload_file, upload_file_action, UploadPath,
    user_login, user_main,
};
use metaforge::model::global_const::{APP_INI, JWT, JWT_EXP};
use metaforge::model::my_request_id::MyMakeRequestId;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    // 如果监听到 ctrl+c 信号就退出应用
    ctrlc::set_handler(|| {
        info!("监听到 CTRL + C 操作, 退出应用程序.");
        std::process::exit(0);
    }).unwrap();

    // 读取服务器 main 参数
    let ini_main: &HashMap<String, String> = APP_INI
        .get("main")
        .ok_or(anyhow!("MAIN section not found"))?;

    // release模式下，日志输出到文件
    #[cfg(not(debug_assertions))]
        let ini_main_mn_log_path = ini_main
        .get("mn_log_path")
        .ok_or(anyhow!("MN_LOG_PATH not found"))?;
    #[cfg(not(debug_assertions))]
        let ini_main_mn_log_name = ini_main
        .get("mn_log_name")
        .ok_or(anyhow!("MN_LOG_NAME not found"))?;
    #[cfg(not(debug_assertions))]
        let (my_writer, _worker_guard): (NonBlocking, WorkerGuard) =
        get_my_tracing_file_writer(ini_main_mn_log_path, ini_main_mn_log_name);

    // debug模式下，日志输出到标准输出
    #[cfg(debug_assertions)]
        let my_writer: fn() -> std::io::Stdout = get_my_tracing_stdout_writer();

    // 初始化日志等级、日志输出位置、日志格式(定制和筛选日志)
    let ini_log_level = ini_main.get("mn_log_level").map(|s| s.to_uppercase());
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
        .event_format(get_my_tracing_format())
        .init();

    // 获取配置文件中的 MYSQL_01 配置信息
    let ini_mysql_01 = APP_INI
        .get("mysql_01")
        .ok_or(anyhow!("{} section not found", "MYSQL_01"))?;
    // 初始化 MYSQL_01 数据库连接池
    let test_mysql_db_01_pool: GrMySQLPool<TestMySqlDb01> =
        init_mysql_conn_pool::<TestMySqlDb01>(ini_mysql_01).await?;

    // 获取配置文件中的 REDIS_01 配置信息
    let ini_redis_01 = APP_INI
        .get("redis_01")
        .ok_or(anyhow!("REDIS_01 section not found"))?;
    // 初始化 REDIS_01 数据库连接池
    let redis_01_pool: Pool = init_redis_conn_pool("REDIS_01", ini_redis_01).await?;

    // 创建请求到服务之间的路由 router
    let x_request_id = HeaderName::from_static("x-request-id"); // 全局请求 ID 的请求头名称
    let router = Router::new()
        .nest_service(
            "/",
            get_service(ServeDir::new("www")).handle_error(|err| async move {
                (
                    StatusCode::NOT_FOUND,
                    format!("处理静态资源出错：{:?}", err),
                )
            }),
        )
        // .nest_service(
        .route("/index", get(index))
        // mysql数据库批量查询
        .route("/api/mysql_qry", post(mysql_query))
        // mysql数据库批量事务
        .route("/api/mysql_trans", post(mysql_transaction))
        // 使用用户名密码获取 jwt token
        .route("/api/get_jwt_token", get(get_jwt_token))
        // 使用有效的 jwt token 可访问受保护的内容
        .route("/get_protected_content", get(get_protected_content))
        // 本站页面跳转
        .route("/redirect01", get(redirect01))
        // 外站页面跳转
        .route("/redirect02", get(redirect02))
        // Session 用户登录页面 & 登录动作
        .route("/login", get(user_login))
        .route("/api/login", post(login_action))
        // Session 用户首页页面
        .route("/main", get(user_main))
        // Session 用户登出动作
        .route("/api/logout", get(logout_action))
        // 文件上传页面
        .route("/uploadfile", get(upload_file))
        .route(
            "/api/uploadfile",
            post(upload_file_action.layer(ConcurrencyLimitLayer::new(5))),
        )
        // 路由中间件配置
        .layer(
            ServiceBuilder::new()
                // 禁用请求体大小默认2MB的限制
                .layer(DefaultBodyLimit::disable())
                // 限制请求体大小为 100MB
                .layer(RequestBodyLimitLayer::new(100 * 1024 * 1024))
                // 在多个请求间共享MySQL01数据库连接池
                .layer(Extension(test_mysql_db_01_pool.clone()))
                // 在多个请求间共享Redis01数据库连接池
                .layer(Extension(RedisPool::<Redis01>::new(redis_01_pool)))
                // 在多个请求间共享文件上传后在服务器上的保存路径
                .layer(Extension(UploadPath {
                    upload_path: ini_main
                        .get("mn_upload_path")
                        .ok_or(anyhow!("获取文件上传路径出错."))?
                        .to_string(),
                }))
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
                        .allow_headers([HeaderName::from_static("authorization")])
                        .allow_origin(Any),
                ),
        );

    // GRPC 服务
    let host = ini_main
        .get("mn_grpc_host")
        .map_or("0.0.0.0", |h| h)
        .to_string();
    let port = ini_main
        .get("mn_grpc_port")
        .map_or("29029", |p| p)
        .to_string();
    let grpc_thread: JoinHandle<Result<(), String>> = tokio::task::spawn(async move {
        // 启动 grpc 服务
        let addr = format!("{host}:{port}");
        info!("GRPC 服务器启动成功: http://{addr}");

        let calculater_srv = Calculator;
        let login_srv = Login {
            jwt: JWT.clone(),
            jwt_exp: JWT_EXP.clone(),
            db_pool: test_mysql_db_01_pool.clone(),
        };
        let jwt_srv = Jwt {
            jwt: JWT.clone(),
            jwt_exp: JWT_EXP.clone(),
        };

        tonic::transport::Server::builder()
            .add_service(CalculatorServiceServer::with_interceptor(
                calculater_srv,
                grpc_check_auth,
            ))
            .add_service(JwtServiceServer::with_interceptor(
                jwt_srv,
                grpc_check_auth,
            ))
            .add_service(LoginServiceServer::new(
                login_srv,
            ))
            .serve(
                addr.parse()
                    .map_err(|err| format!("GRPC 服务器地址解析失败：{:?}", err))?,
            )
            .await
            .map_err(|err| format!("GRPC 服务器启动失败：{:?}", err))?;

        Ok(())
    });

    // HTTP 服务
    let host = ini_main
        .get("mn_http_host")
        .map_or("127.0.0.1", |h| h)
        .to_string();
    let port = ini_main
        .get("mn_http_port")
        .map_or("5000", |p| p)
        .to_string();
    let web_thread: JoinHandle<Result<(), String>> = tokio::task::spawn(async move {
        // 启动 Http 服务
        let addr = format!("{host}:{port}");
        info!("Http 服务器启动成功: http://{addr}");

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|err| format!("Http 服务器监听端口失败：{:?}", err))?;
        axum::serve(listener, router)
            .await
            .map_err(|err| format!("Http 服务器启动失败：{:?}", err))?;
        Ok(())
    });

    // 启动 GRPC 和 HTTP 服务
    let (_, _) = (
        grpc_thread.await.map_err(|err| anyhow!(err))?,
        web_thread.await.map_err(|err| anyhow!(err))?,
    );

    // 退出应用程序
    Ok(())
}
