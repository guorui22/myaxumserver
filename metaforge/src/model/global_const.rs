use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::auth::JwtSecret;
use crate::config::init_server_config;

lazy_static! {

    /// 读取服务器配置文件参数信息
    pub static ref APP_INI: HashMap<String,HashMap<String,String>> = init_server_config().expect("Failed to read server config file");

    /// JWT 初始化
    pub static ref JWT: JwtSecret = {
        let jwt_map = APP_INI.get("jwt").expect("failed to read jwt section");
        let jwt_secret = jwt_map.get("jwt_secret").expect("failed to read jwt_secret");
        let jwt_iss = jwt_map.get("jwt_iss").expect("failed to read jwt_iss");
        let jwt = JwtSecret::new(jwt_secret.to_string(), jwt_iss.to_string());
        jwt
    };

    /// JWT_EXP 默认过期时长
    pub static ref JWT_EXP: i64 = {
        let jwt_map = APP_INI.get("jwt").expect("jwt section not found");
        let jwt_exp: i64 = jwt_map.get("jwt_exp").expect("jwt_exp not found").parse().expect("failed to parse jwt_exp");
        jwt_exp
    };

    /// gRPC 配置参数
    pub static ref GRPC_HOST: String = {
        let main_map = APP_INI.get("main").expect("main section not found");
        let mn_grpc_host: String = main_map.get("mn_grpc_host").expect("mn_grpc_host not found").parse().expect("failed to parse mn_grpc_host");
        mn_grpc_host
    };
    pub static ref GRPC_PORT: String = {
        let main_map = APP_INI.get("main").expect("main section not found");
        let mn_grpc_port: String = main_map.get("mn_grpc_port").expect("mn_grpc_port not found").parse().expect("failed to parse mn_grpc_port");
        mn_grpc_port
    };

}
