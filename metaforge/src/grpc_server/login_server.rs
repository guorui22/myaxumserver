use libproto::login_service_server::LoginService;
use libproto::{LoginReply, LoginReplyData, LoginRequest};
use tonic::{Request, Response, Status};
use crate::auth::JwtSecret;

#[derive(Clone, Debug)]
pub struct Login<T: Clone + Send + Sync + 'static> {
    pub jwt: JwtSecret,
    pub jwt_exp: i64,
    pub db_pool: T,
}

#[tonic::async_trait]
impl<T: Clone + Send + Sync + 'static> LoginService for Login<T> {
    async fn do_login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        let input = request.into_inner();
        let usercode = input.usercode;
        let password = input.password;

        dbg!(format!("usercode: {}, password: {}", usercode, password));

        let jwt = &self.jwt;
        let claims = jwt.create_claims(usercode.clone(), "郭睿".to_string(), self.jwt_exp);
        let token = jwt.to_token(&claims).map_err(|err| Status::internal(err.to_string()))?;

        let output_data = LoginReplyData {
            usercode: Some(usercode),
            username: Some("郭睿".to_string()),
            jwt: Some(token),
        };
        let output = LoginReply {
            status: 0,
            message: "success".to_string(),
            data: Some(output_data),
        };
        Ok(tonic::Response::new(output))
    }
}
