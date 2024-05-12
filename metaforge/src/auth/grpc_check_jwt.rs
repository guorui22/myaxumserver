use tonic::{Request, Status};
use crate::model::global_const::JWT;

/// 检查请求头中的 token 是否正确
pub fn grpc_check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(t) if {
            let jwt = t.to_str().map_err(|_|Status::unauthenticated("No valid auth token"))?.split_whitespace().collect::<Vec<&str>>()[1];
            let claims = JWT.verify_and_get(jwt).unwrap();
            !claims.is_expired()
        } => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

