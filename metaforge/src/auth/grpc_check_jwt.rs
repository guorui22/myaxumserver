use tonic::{Request, Status};
use crate::model::global_const::JWT;

/// 检查请求头中的 token 是否正确
pub fn grpc_check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(t) if {
            let jwt = t.to_str().map_err(|_|Status::unauthenticated("不是有效的 JWT Token。"))?.split_whitespace().collect::<Vec<&str>>()[1];
            dbg!(format!("request jwt: {}", jwt));
            let claims = JWT.verify_and_get(jwt).map_err(|_|Status::unauthenticated("JWT Token 验证失败。"))?;
            if claims.is_expired() {
                return Err(Status::unauthenticated("JWT Token 已过期。"));
            } else {
                true
            }
        } => Ok(req),
        _ => Err(Status::unauthenticated("请求头中没有 authorization 字段。")),
    }
}

