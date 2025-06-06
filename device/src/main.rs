use std::env;
use std::{cell::RefCell, rc::Rc};

mod logger;
mod user_interface;

use logger::logger_msg::*;
use user_interface::input::input_controller::InterruptCapable;
use user_interface::input::{Key, KeyboardController};

use user_interface::output::terminal::{OutputRenderer, Renderer};

fn render_loop(renderer: &mut Renderer) -> anyhow::Result<()> {
    for i in 0..20 {
        renderer.clear()?;
        renderer.draw_box(5, 3, 30, 10)?;
        renderer.draw_text(7, 5, "Moving counter:")?;
        renderer.draw_text(7, 6, &format!("Frame: {}", i))?;

        let anim = "•".repeat((i % 10 + 1) as usize);
        renderer.draw_text(7, 7, &anim)?;

        renderer.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}

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

    let mut renderer = Renderer::new();

    if let Err(e) = render_loop(&mut renderer) {
        event!(Level::ERROR, error = ?e, "Failed to render.");
    }

    keyboard_controller.borrow_mut().run();
}
