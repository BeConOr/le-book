pub mod logger_msg;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init_logger(logfile_name: &str) {
    let logfile = RollingFileAppender::builder()
        .rotation(Rotation::NEVER)
        .filename_prefix(logfile_name)
        .filename_suffix("log")
        .max_log_files(10)
        .build("log")
        .expect("failed to initialize rolling file appender");
    let stdout = std::io::stdout.with_max_level(logger_msg::Level::INFO);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    let fmt_layer = fmt::layer()
        .with_writer(stdout.and(logfile))
        .with_ansi(false)
        .with_target(false)
        .with_thread_names(true)
        .with_line_number(true)
        .with_file(true);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .with(env_filter)
        .init();
}
