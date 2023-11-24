use std::collections::HashMap;

use axum::BoxError;
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use config::Config;
use tracing::info;

/// 读取服务器配置文件参数信息
pub fn init_server_config() -> Result<HashMap<String, HashMap<String, String>>, String> {
    // 获取配置文件路径
    let root_path =
        std::env::current_dir().map_err(|err| format!("{}", err))?;

    #[cfg(not(windows))]
        let path_to_ini = root_path.join("conf").join("conf.toml");

    #[cfg(windows)]
        let path_to_ini = root_path.join("conf_windows.toml");

    Config::builder()
        .add_source(config::File::from(path_to_ini))
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

/// 获取请求头中的 x-request-id
pub fn get_request_id(headers: &HeaderMap) -> String {
    headers
        .get("x-request-id")
        .map_or("".to_string(), |x| x.to_str().unwrap().to_string())
}

/// 通用错误处理提取器
pub async fn handle_layer_error(
    // `Method` and `Uri` are extractors so they can be used here
    method: Method,
    uri: Uri,
    // the last argument must be the error itself
    err: BoxError
) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("`{} {}` failed with {}", method, uri, err),
    )
}
