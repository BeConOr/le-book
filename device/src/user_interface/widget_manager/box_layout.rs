use std::cell::RefCell;
use std::rc::Rc;

use super::layout::{Layout, widget};

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
        self.children.push(widget);
        self.reflow();
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

        let prev_box = cur_box;

        Self {
            cur_box,
            prev_box,
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
        let mut offset_x = self.cur_box.x;
        let mut offset_y = self.cur_box.y;

        let mut total_width = 0;
        let mut total_height = 0;
        let mut max_cross = 0;

        for child in &self.children {
            let mut w = child.borrow_mut();
            match self.direction {
                Direction::Vertical => {
                    w.set_position(self.cur_box.x, offset_y);
                    offset_y += w.height() + self.spacing;
                    total_height += w.height() + self.spacing;
                    max_cross = max_cross.max(w.width());
                }
                Direction::Horizontal => {
                    w.set_position(offset_x, self.cur_box.y);
                    offset_x += w.width() + self.spacing;
                    total_width += w.width() + self.spacing;
                    max_cross = max_cross.max(w.height());
                }
            }
        }

        match self.direction {
            Direction::Vertical => {
                self.cur_box.height = total_height.saturating_sub(self.spacing);
                self.cur_box.width = max_cross;
            }
            Direction::Horizontal => {
                self.cur_box.width = total_width.saturating_sub(self.spacing);
                self.cur_box.height = max_cross;
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}
