// debug模式下，日志输出到标准输出
#[cfg(debug_assertions)]
use std::io::Stdout;
#[cfg(not(debug_assertions))]
use std::path::Path;

use chrono::Local;
// release模式下，日志输出到文件
#[cfg(not(debug_assertions))]
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt::format::{Format, Full, Writer};
use tracing_subscriber::fmt::time::FormatTime;

/// 用来自定义日志中的时间格式
/// 类似 2023-11-17 09:48:50.616722293 +08:00 格式
pub struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%F %T%.9f %:z"))
    }
}

/// 获取日志输出格式
pub fn get_my_format() -> Format<Full, LocalTimer> {
    // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置(main.rs、lib.rs两文件中的日志输出只显示crate名称，其他*.rs文件的日志输出就会显示文件名称)、线程ID、线程名称、日志产生的源代码行号、日志的时间格式
    // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(
            #[cfg(debug_assertions)]
            true,
            #[cfg(not(debug_assertions))]
            false,
        )
        .with_line_number(true)
        .with_timer(LocalTimer)
}

#[cfg(debug_assertions)]
pub fn get_my_stdout_writer() -> fn() -> Stdout {
    std::io::stdout
}

#[cfg(not(debug_assertions))]
pub fn get_my_file_writer(
    directory: impl AsRef<Path>,
    file_name_prefix: impl AsRef<Path>,
) -> (NonBlocking, WorkerGuard) {
    // 使用tracing_appender，指定日志的输出目标位置
    // 参考: https://docs.rs/tracing-appender/0.2.0/tracing_appender/
    let file_appender = tracing_appender::rolling::daily(directory, file_name_prefix);
    tracing_appender::non_blocking(file_appender)
}
