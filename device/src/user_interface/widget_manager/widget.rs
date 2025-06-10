use crate::user_interface::output::draw;

pub trait Widget {
    fn render(&mut self) {
        self.clear();
        self.raw_render();
        draw::flush();
    }

    fn clear(&mut self) {
        let prev_box = self.prev_geometry();
        draw::clear_box(prev_box.x, prev_box.y, prev_box.width, prev_box.height);
    }

    fn raw_render(&mut self);

    fn set_position(&mut self, x: u16, y: u16);
    fn set_size(&mut self, width: u16, height: u16);
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn prev_geometry(&self) -> Box<u16>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Box<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: Default> Default for Box<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            width: T::default(),
            height: T::default(),
        }
    }
}
