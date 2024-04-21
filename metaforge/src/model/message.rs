use serde_json::Value;
use tokio::sync::oneshot::Sender;

pub struct JsRsMsg {
    pub sender: Sender<Value>,
    pub js_name: String,
    pub js_method_name: String,
    pub js_method_args: Value,
}
