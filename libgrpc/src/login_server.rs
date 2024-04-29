use libproto::login_service_server::LoginService;
use libproto::{LoginReply, LoginReplyData, LoginRequest};
use tonic::{Request, Response, Status};
use libauth::Jwt;

pub struct Login{
    pub jwt: Jwt,
    pub jwt_exp: i64,
}

#[tonic::async_trait]
impl LoginService for Login {
    async fn do_login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {

        let input = request.into_inner();
        let jwt = &self.jwt;
        let claims = jwt.new_claims(input.usercode.clone(), "郭睿".to_string(), self.jwt_exp);
        let token = jwt.token(&claims).map_err(|err| Status::internal(err.to_string()))?;

        let output_data = LoginReplyData {
            usercode: Some(input.usercode),
            username: Some("郭睿".to_string()),
            jwt: Some(token)
        };
        let output = LoginReply {
            status: 0,
            message: "success".to_string(),
            data: Some(output_data),
        };
        Ok(tonic::Response::new(output))
    }
}
