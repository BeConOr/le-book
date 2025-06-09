pub use super::layout::{Layout, widget};

use std::cell::RefCell;
use std::rc::Rc;

pub struct VBoxLayout {
    pub x: u16,
    pub y: u16,
    pub spacing: u16,
    pub children: Vec<Rc<RefCell<dyn widget::Widget>>>,
    current_y: u16,
    width: u16,
}

impl widget::Widget for VBoxLayout {
    fn render(&mut self) {
        for child in &self.children {
            child.borrow_mut().render();
        }
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    #[allow(unused)]
    fn set_size(&mut self, width: u16, height: u16) {}

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.current_y
    }
}

impl Layout for VBoxLayout {
    fn add_widget(&mut self, widget: Rc<RefCell<dyn widget::Widget>>) {
        self.set_widget_position(&widget);
        self.children.push(widget);
    }
}

impl VBoxLayout {
    fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
            spacing: 0,
            children: Vec::new(),
            current_y: 0,
            width: 0,
        }
    }

    pub fn set_spacing(&mut self, spacing: u16) {
        self.spacing = spacing;
        self.reflow();
    }

    pub fn reflow(&mut self) {
        let children = self.children.clone();
        self.current_y = 0;
        self.width = 0;
        for child in &children {
            self.set_widget_position(child);
        }
    }

    fn set_widget_position(&mut self, widget: &Rc<RefCell<dyn widget::Widget>>) {
        let mut w = widget.borrow_mut();
        w.set_position(self.x, self.y + self.current_y);
        self.current_y += self.spacing + w.height();
        self.width = self.width.max(w.width());
    }
}
