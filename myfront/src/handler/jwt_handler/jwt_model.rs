use serde::Deserialize;

/// 客户端为获取授权 token 而向服务器提交信息的结构体
/// client_id       客户唯一ID
/// client_pwd   客户密码
#[derive(Debug, Deserialize)]
pub struct AuthInfo {
    pub client_code: String,
    pub client_pwd: String,
}
