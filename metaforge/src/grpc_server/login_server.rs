use axum::Json;
use libproto::login_service_server::LoginService;
use libproto::{LoginReply, LoginReplyData, LoginRequest};
use tonic::{Request, Response, Status};
use libdatabase::{GrMySQLPool, TestMySqlDb01};
use crate::auth::{aes_encrypt, JwtSecret};

#[derive(Clone, Debug)]
pub struct Login {
    pub jwt: JwtSecret,
    pub jwt_exp: i64,
    pub db_pool: GrMySQLPool<TestMySqlDb01>,
}

#[tonic::async_trait]
impl LoginService for Login {
    async fn do_login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        let input = request.into_inner();
        let usercode = input.usercode;
        let password = input.password;

        dbg!(format!("usercode: {}, password: {}", usercode, password));

        let user = libdatabase::mysql_query(self.db_pool.clone(), vec![format!("select * from sys_user where user_code = '{}' and user_password = '{}'", usercode, aes_encrypt(password))])
            .await
            .map_err(|err| Status::internal(format!("数据库查询失败: {}", err)))
            .map(Json)?;
        let status = Status::internal("数据库查询结果解析失败。".to_string());
        dbg!(format!("user: {:?}", user.as_object().ok_or(status.clone())?
            .get("result").ok_or(status.clone())?
            .as_array().ok_or(status.clone())?
            .get(0).ok_or(status.clone())?));
        dbg!(format!("user: {:?}", user));

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
