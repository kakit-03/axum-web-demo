pub mod config;
pub mod entity;
pub mod middleware;
pub mod err;
pub mod router;
pub mod state;
pub mod service;
pub mod dto;
pub mod util;
pub mod vo;

use crate::config::Config;
use chrono::Local;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
pub use err::{AppError, AppErrorType};

pub type Result<T> = std::result::Result<T, crate::AppError>;


// 格式化日志的输出时间格式
// 格式化日志的输出时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub fn init(cfg: &Config) -> WorkerGuard {
    // 直接初始化，采用默认的Subscriber，默认只输出INFO、WARN、ERROR级别的日志
    // tracing_subscriber::fmt::init();

    let level = if cfg.app.debug == true {
        Level::DEBUG
    } else {
        Level::INFO
    };
    // guard必须返回到main()函数中，否则不输出任何信息到日志文件
    let (non_blocking, guard) = if cfg.app.env == "dev" {
        // 开发环境，日志输出到控制台
        tracing_appender::non_blocking(std::io::stdout())
    } else {
        // 使用tracing_appender，指定日志的输出目标位置
        // 参考: https://docs.rs/tracing-appender/latest/tracing_appender/index.html
        tracing_appender::non_blocking(tracing_appender::rolling::daily(
            &cfg.log.path,
            "tracing.log",
        ))
    };
    // 初始化并设置日志格式(定制和筛选日志)
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_file(true)
        .with_line_number(true) // 写入标准输出
        .with_ansi(false) // 关掉ansi的颜色输出功能
        .with_timer(LocalTimer)
        .with_writer(non_blocking)
        .json()
        .flatten_event(true)
        .init(); // 初始化并将SubScriber设置为全局SubScriber
    guard
}