use askama::Template;
use serde::Deserialize;

/// 文件上传目录结构体
#[derive(Debug, Clone, Deserialize)]
pub struct UploadPath {
    pub upload_path: String,
}

/// 文件上传页模板
#[derive(Template)]
#[template(path = "upload_file.html")]
pub struct UploadFileTemplate;
