use libproto::login_service_server::LoginService;
use libproto::{LoginReply, LoginReplyData, LoginRequest};
use tonic::{Request, Response, Status};
use libdatabase::{GrMySQLPool, TestMySqlDb01};
use crate::auth::{aes_encrypt, JwtSecret};
use tracing::info;

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

        // 获取请求参数，用户名和密码
        let input = request.into_inner();
        let usercode = input.usercode;
        let password = input.password;

        // 查询数据库，验证用户名和密码
        let query_result = libdatabase::mysql_query(self.db_pool.clone(), vec![format!("select * from sys_user where user_code = '{}' and user_password = '{}'", usercode, aes_encrypt(password))])
            .await
            .map_or_else(|err| Err(Status::internal(err.to_string())), |result| Ok(result))?;

        // 解析查询结果
        let users = query_result.as_object().ok_or_else(||Status::internal("查询结果解析失败。".to_string()))?
            .get("result").ok_or_else(||Status::internal("查询结果解析失败。".to_string()))?
            .as_array().ok_or_else(||Status::internal("查询结果解析失败。".to_string()))?
            .get(0).ok_or_else(||Status::internal("查询结果解析失败。".to_string()))?
            .as_array().ok_or_else(||Status::internal("查询结果解析失败。".to_string()))?;

        // 如果返回结果不唯一，说明用户名或密码错误
        if users.len() != 1 {
            return Err(Status::unauthenticated("用户名或密码错误".to_string()));
        }

        // 获取登录用户信息
        let user_for_login = users.get(0)
            .ok_or(Status::internal("获取登录用户失败。".to_string()))?
            .as_object().ok_or(Status::internal("登录用户解析失败。".to_string()))?;

        // 打印登录用户信息
        info!("user for login: {}", serde_json::to_string(&user_for_login).map_err(|err| Status::internal(err.to_string()))?);

        // 获取用户名称
        let user_name = user_for_login.get("user_name").map_or("".to_string(), |name| {
            name.to_string()
        });

        // 生成 JWT Token
        let jwt = &self.jwt;
        let claims = jwt.create_claims(usercode.clone(), user_name.clone(), self.jwt_exp);
        let token = jwt.to_token(&claims).map_err(|err| Status::internal(err.to_string()))?;

        // 返回登录结果
        let output_data = LoginReplyData {
            usercode: Some(usercode),
            username: Some(user_name),
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
