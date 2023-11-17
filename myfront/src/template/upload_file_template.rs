use askama::Template;

/// 文件上传页模板
#[derive(Template)]
#[template(path = "upload_file.html")]
pub struct UploadFileTemplate;
