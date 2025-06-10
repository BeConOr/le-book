pub use super::layout::widget;

use crate::user_interface::output::draw;

const BORDER_PADDING: u16 = 2 * draw::BORDER_SIZE;
const LABEL_HEIGHT: u16 = 1 + BORDER_PADDING;

pub struct Label {
    text: String,
    cur_box: widget::Box<u16>,
    prev_box: widget::Box<u16>,
}

impl Label {
    pub fn new(text: &str) -> Self {
        let width = Label::get_label_length(text);
        let height = LABEL_HEIGHT;
        let cur_box = widget::Box::<u16> {
            x: 0,
            y: 0,
            width,
            height,
        };
        let prev_box = cur_box;

        Self {
            text: text.to_string(),
            cur_box,
            prev_box,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.cur_box.width = Label::get_label_length(text);
    }

    fn get_label_length(text: &str) -> u16 {
        text.len() as u16 + BORDER_PADDING
    }
}

impl widget::Widget for Label {
    fn raw_render(&mut self) {
        self.prev_box = self.cur_box;
        draw::draw_text(
            self.cur_box.x + draw::BORDER_SIZE,
            self.cur_box.y + draw::BORDER_SIZE,
            &self.text,
        );
        draw::draw_box(
            self.cur_box.x,
            self.cur_box.y,
            self.cur_box.width,
            self.cur_box.height,
        );
        draw::flush();
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.cur_box.x = x;
        self.cur_box.y = y;
    }

    #[allow(unused)]
    fn set_size(&mut self, width: u16, height: u16) {}

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
