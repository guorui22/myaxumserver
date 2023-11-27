use std::collections::HashMap;

use config::Config;
use tracing::info;

/// 读取服务器配置文件参数信息
pub fn init_server_config() -> Result<HashMap<String, HashMap<String, String>>, String> {
    // 获取配置文件路径
    let root_path =
        std::env::current_dir().map_err(|err| format!("{}", err))?;

    #[cfg(not(windows))]
        let path_to_conf = root_path.join("conf").join("conf.toml");

    #[cfg(windows)]
        let path_to_conf = root_path.join("conf_windows.toml");

    Config::builder()
        .add_source(config::File::from(path_to_conf))
        .build()
        .map_err(|err| format!("{}", err))?
        .try_deserialize::<HashMap<String, HashMap<String, String>>>()
        .map_err(|err| format!("{}", err))
}


/// 监听 ctrl+c 信号退出应用
pub fn watch_ctrl_c_to_exit() {
    ctrlc::set_handler(|| {
        info!("Received CTRL + C, Quit Application.");
        std::process::exit(0);
    })
    .unwrap_or_else(|err| {
        panic!(
            "{}",
            err.to_string()
        )
    });
}
