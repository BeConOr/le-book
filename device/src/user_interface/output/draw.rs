use super::output_renderer::OutputRenderer;
use super::terminal::Renderer;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub use super::terminal::BORDER_SIZE;

static RENDERER: Lazy<Mutex<Renderer>> = Lazy::new(|| Mutex::new(Renderer::new()));

fn with_renderer<F, R>(f: F) -> R
where
    F: FnOnce(&mut Renderer) -> R,
{
    let mut renderer = RENDERER.lock().unwrap();
    f(&mut renderer)
}

pub fn draw_text(x: u16, y: u16, text: &str) {
    with_renderer(|r| r.draw_text(x, y, text).unwrap());
}

pub fn draw_box(x: u16, y: u16, width: u16, height: u16) {
    with_renderer(|r| r.draw_box(x, y, width, height).unwrap());
}

pub fn flush() {
    with_renderer(|r| r.flush().unwrap());
}

pub fn clear() {
    with_renderer(|r| r.clear().unwrap());
}

pub fn clear_box(x: u16, y: u16, width: u16, height: u16) {
    with_renderer(|r| r.clear_box(x, y, width, height).unwrap());
}
