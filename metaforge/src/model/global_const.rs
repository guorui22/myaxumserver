use std::collections::HashMap;
use anyhow::anyhow;
use lazy_static::lazy_static;
use libconfig::init_server_config;
use crate::auth::Jwt;

lazy_static! {

    /// 读取服务器配置文件参数信息
    pub static ref APP_INI: HashMap<String,HashMap<String,String>> = init_server_config().expect("Failed to read server config file");

    /// JWT 初始化
    pub static ref JWT: Jwt = {
        let jwt_map = APP_INI.get("jwt").expect("Failed to read JWT section");
        let jwt_secret = jwt_map.get("jwt_secret").expect("Failed to read JWT_SECRET");
        let jwt_iss = jwt_map.get("jwt_iss").expect("Failed to read JWT_ISS");
        let jwt = Jwt::new(jwt_secret.to_string(), jwt_iss.to_string());
        jwt
    };

    /// JWT_EXP 默认过期时长
    pub static ref JWT_EXP: i64 = {
        let jwt_map = APP_INI.get("jwt").expect("JWT section not found");
        let jwt_exp: i64 = jwt_map.get("jwt_exp").expect("JWT_EXP not found").parse().expect("Failed to parse JWT_EXP");
        jwt_exp
    };

}
