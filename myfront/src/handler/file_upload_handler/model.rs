use serde::Deserialize;

/// 文件上传目录结构体
#[derive(Debug, Clone, Deserialize)]
pub struct UploadPath {
    pub upload_path: String,
}