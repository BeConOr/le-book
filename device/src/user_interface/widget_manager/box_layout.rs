use super::layout::{Layout, widget};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub enum Direction {
    Vertical,
    Horizontal,
}

pub struct GBoxLayout {
    cur_box: widget::Box<u16>,
    prev_box: widget::Box<u16>,
    spacing: u16,
    direction: Direction,
    pub children: Vec<Rc<RefCell<dyn widget::Widget>>>,
}

impl widget::Widget for GBoxLayout {
    fn raw_render(&mut self) {
        self.prev_box = self.cur_box;
        for child in &self.children {
            child.borrow_mut().raw_render();
        }
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.cur_box.x = x;
        self.cur_box.y = y;
        self.reflow();
    }

    fn set_size(&mut self, _width: u16, _height: u16) {}

    fn width(&self) -> u16 {
        self.cur_box.width
    }

    fn height(&self) -> u16 {
        self.cur_box.height
    }

    fn prev_geometry(&self) -> widget::Box<u16> {
        self.prev_box
    }
}

impl Layout for GBoxLayout {
    fn add_widget(&mut self, widget: Rc<RefCell<dyn widget::Widget>>) {
        self.set_widget_position(&widget);
        self.children.push(widget);
    }
}

impl GBoxLayout {
    pub fn new(x: u16, y: u16, direction: Direction) -> Self {
        let cur_box = widget::Box::<u16> {
            x,
            y,
            width: 0,
            height: 0,
        };
        Self {
            cur_box,
            prev_box: cur_box,
            spacing: 0,
            direction,
            children: Vec::new(),
        }
    }

    pub fn set_spacing(&mut self, spacing: u16) {
        self.spacing = spacing;
        self.reflow();
    }

    pub fn reflow(&mut self) {
        let children = self.children.clone();
        self.cur_box.width = 0;
        self.cur_box.height = 0;
        for child in &children {
            self.set_widget_position(child);
        }
    }

    fn set_widget_position(&mut self, widget: &Rc<RefCell<dyn widget::Widget>>) {
        let mut w = widget.borrow_mut();
        match self.direction {
            Direction::Vertical => {
                w.set_position(self.cur_box.x, self.cur_box.y + self.cur_box.height);
                self.cur_box.height += self.spacing + w.height();
                self.cur_box.width = self.cur_box.width.max(w.width());
            }
            Direction::Horizontal => {
                w.set_position(self.cur_box.x + self.cur_box.width, self.cur_box.y);
                self.cur_box.width += self.spacing + w.width();
                self.cur_box.height = self.cur_box.height.max(w.height());
            }
        }
    }
}
