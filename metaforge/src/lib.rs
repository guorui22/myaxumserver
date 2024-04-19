use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

pub mod handler;

// #[derive(Serialize, Deserialize)]
pub struct MyArgs {
    pub sender: Sender<String>,
    pub msg: String,
}
