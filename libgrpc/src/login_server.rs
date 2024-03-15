use libproto::login_service_server::LoginService;
use libproto::{LoginReply, LoginRequest};
use tonic::{Request, Response, Status};

pub struct Login;

#[tonic::async_trait]
impl LoginService for Login {
    async fn do_login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        let input = request.into_inner();
        let output = LoginReply {
            status: 0,
            message: "success".to_string(),
            usercode: Some(input.usercode),
            username: Some("郭睿".to_string()),
        };
        Ok(tonic::Response::new(output))
    }
}
