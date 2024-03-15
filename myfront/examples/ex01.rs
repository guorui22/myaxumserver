use axum::{middleware, routing::get, Router};
use myfront::middleware::print_middle_ware;

#[tokio::main]
async fn main() -> Result<(), String> {
    // 创建请求到服务之间的路由 router
    let router = Router::new()
        .route("/", get(|| async { "Hello, World! 01" }))
        .route(
            "/h",
            get(|| async { "Hello, World! 02" }).layer(middleware::from_fn(print_middle_ware)),
        );

    // 启动应用监听本地 3000 端口
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3300")
        .await
        .unwrap();
    axum::serve(listener, router)
        .await
        .map_err(|err| format!("服务启动失败：{:?}", err))?;
    Ok(())
}
