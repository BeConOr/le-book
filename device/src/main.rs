use std::env;

mod logger;
use logger::*;

fn main() {
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let app_name = exe_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown");

    init_logger(app_name);
}
