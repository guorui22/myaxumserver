use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

pub async fn print_middle_ware(request: Request, next: Next) -> Result<Response, StatusCode> {
    println!("print_middle_ware for test.");

    let response = next.run(request).await;
    Ok(response)
}
