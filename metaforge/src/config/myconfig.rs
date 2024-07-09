use std::collections::HashMap;
use anyhow::anyhow;
use config::Config;

/// 读取服务器配置文件参数信息
pub fn init_server_config() -> Result<HashMap<String, HashMap<String, String>>, anyhow::Error> {
    // 获取配置文件路径
    let root_path = std::env::current_dir().map_err(|err| anyhow!(err))?;
    let path_to_conf = root_path.join("conf").join("conf.toml");

    // 读取配置文件
    Config::builder()
        .add_source(config::File::from(path_to_conf))
        .build()
        .map_err(|err| anyhow!(err))?
        .try_deserialize::<HashMap<String, HashMap<String, String>>>()
        .map_err(|err| anyhow!(err))
}
