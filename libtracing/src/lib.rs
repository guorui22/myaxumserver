pub use tracing::debug;
pub use tracing::error;
pub use tracing::info;
pub use tracing::Level;
pub use tracing::trace;
pub use tracing::warn;
pub use tracing_appender;
pub use tracing_subscriber;
pub use tracing_subscriber::fmt::format::{Format, Full, Writer};
pub use tracing_subscriber::fmt::time::FormatTime;

pub use my_tracing::get_my_tracing_format;
pub use my_tracing::get_my_tracing_stdout_writer;
pub use my_tracing::LocalTimer;

mod my_tracing;

