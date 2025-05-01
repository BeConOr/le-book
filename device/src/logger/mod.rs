#[allow(unused_imports)]
pub use tracing::{Level, event, instrument, span};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;

pub fn init_logger(logfile_name: &str) {
    let logfile = RollingFileAppender::builder()
        .rotation(Rotation::NEVER)
        .filename_prefix(logfile_name)
        .filename_suffix("log")
        .max_log_files(10)
        .build("log")
        .expect("failed to initialize rolling file appender");
    let stdout = std::io::stdout.with_max_level(Level::INFO);

    tracing_subscriber::fmt()
        .with_writer(stdout.and(logfile))
        .with_ansi(false)
        .init();
}
