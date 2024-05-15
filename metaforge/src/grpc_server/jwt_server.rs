use tonic::{Request, Response, Status};
use libproto::jwt_service_server::JwtService;
use libproto::{JwtReply, JwtRequest};
use crate::model::global_const::JWT;

pub struct Jwt {
    pub jwt: crate::auth::Jwt,
    pub jwt_exp: i64,
}

#[tonic::async_trait]
impl JwtService for Jwt {
    async fn get_jwt_token(&self, request: Request<JwtRequest>) -> Result<Response<JwtReply>, Status> {
        let input = request.into_inner();
        let claims = JWT.verify_and_get(&input.old_jwt).map_err(|err| Status::unauthenticated(err.to_string()))?;
        Ok(tonic::Response::new(JwtReply { new_jwt: JWT.to_token(&JWT.create_claims_with_expire(claims, self.jwt_exp)).map_err(|err| Status::unauthenticated(format!("JWT Token 生成失败: {}", err)))? }))
    }
}
