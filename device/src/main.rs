use std::env;
use std::{cell::RefCell, rc::Rc};

mod logger;
mod user_interface;

use logger::logger_msg::*;
use user_interface::input::input_controller::InterruptCapable;
use user_interface::input::{Key, KeyboardController};

fn main() {
    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            event!(Level::ERROR, error = ?e, "Failed to get current exe path");
            panic!("Cannot get the app path");
        }
    };
    let app_name = exe_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown");

    logger::init_logger(app_name);

    let keyboard_controller = match KeyboardController::new("/dev/input/event3") {
        Ok(controller) => controller,
        Err(e) => {
            event!(Level::ERROR, error = ?e, "Failed to create controller");
            panic!("Cannot create controller");
        }
    };

    let keyboard_controller = Rc::new(RefCell::new(keyboard_controller));

    let q_key = Key::new(16, keyboard_controller.clone());
    if let Err(e) = q_key.on_change(|ev| {
        event!(Level::INFO, "Q key event: {:?}", ev);
    }) {
        event!(Level::ERROR, error = ?e, "Failed to register a callback");
        panic!("Cannot register a callback");
    }

    keyboard_controller.borrow_mut().run();
}
