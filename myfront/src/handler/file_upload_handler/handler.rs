use std::path::Path;

use askama::Template;
use axum::Extension;
use axum::extract::Multipart;
use axum::http::HeaderMap;
use axum::response::Html;
use axum_macros::debug_handler;
use serde_json::json;
use uuid::Uuid;

use crate::share::UploadPath;
use crate::template::UploadFileTemplate;

/// 文件上传页面
#[debug_handler]
pub async fn upload_file() -> Result<Html<String>, String> {
    let html = UploadFileTemplate.render().map_err(|err| {
        format!("upload_file 模板渲染失败：{}", err)
    })?;
    Ok(Html(html))
}

/// 文件上传操作
pub async fn upload_file_action(
    Extension(UploadPath { upload_path }): Extension<UploadPath>,
    mut multipart: Multipart,
) -> Result<(HeaderMap, String), String> {
    let mut rst = vec![];
    while let Some(file) = multipart.next_field().await.map_err(|err|err.to_string())? {
        let filename = format!("{}-{}", Uuid::new_v4().as_simple(), file.file_name().ok_or("获取文件名称出错。")?); // 上传的文件名
        let upload_path = Path::new(&upload_path).join(&filename); //
        let data = file.bytes().await.map_err(|err| {
            format!("获取上传文件内容失败：{}", err)
        })?; // 上传的文件的内容

        if data.is_empty() {
            continue;
        }

        // 保存上传的文件
        tokio::fs::write(upload_path, &data)
            .await
            .map_err(|err|format!("保存上传文件到磁盘失败：{}", err))?;

        // 构造返回结果
        rst.push(format!(
            "【上传的文件】文件名：{:?}, 文件大小：{}",
            filename,
            data.len()
        ));
    }
    if rst.is_empty(){
        rst.push(String::from("没有上传文件"))
    }

    cn(json!(rst).to_string()).await

}

/// 中文响应
async fn cn(msg: String) -> Result<(HeaderMap, String), String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/plain;charset=utf-8".parse().map_err(|err| {
            format!("设置 Content-Type 失败：{}", err)
        })?,
    );
    Ok((headers, msg))
}