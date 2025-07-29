pub use super::layout::{Layout, widget};

use super::box_layout::{Direction, GBoxLayout};
use delegate;
use std::cell::RefCell;
use std::rc::Rc;

pub struct VBoxLayout {
    layout: GBoxLayout,
}

impl VBoxLayout {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            layout: GBoxLayout::new(x, y, Direction::Vertical),
        }
    }

    pub fn set_spacing(&mut self, spacing: u16) {
        self.layout.set_spacing(spacing);
    }
}

impl widget::Widget for VBoxLayout {
    delegate::delegate! {
        to self.layout {
            fn raw_render(&mut self);
            fn set_position(&mut self, x: u16, y: u16);
            fn set_size(&mut self, width: u16, height: u16);
            fn width(&self) -> u16;
            fn height(&self) -> u16;
            fn prev_geometry(&self) -> widget::Box<u16>;
        }
    }
}

impl Layout for VBoxLayout {
    fn add_widget(&mut self, widget: Rc<RefCell<dyn widget::Widget>>) {
        self.layout.add_widget(widget);
    }
}
