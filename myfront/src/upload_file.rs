use askama::Template;
use axum::headers::HeaderMap;

/// 允许上传的大小
pub const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 10; // 10MB

/// 用户首页模板
#[derive(Template)]
#[template(path = "upload_file.html")]
pub struct UploadFileTemplate {}

/// 中文响应
pub async fn cn(msg: String) -> Result<(HeaderMap, String), String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/plain;charset=utf-8".parse().map_err(|err| {
            format!("设置 Content-Type 失败：{}", err)
        })?,
    );
    Ok((headers, msg))
}