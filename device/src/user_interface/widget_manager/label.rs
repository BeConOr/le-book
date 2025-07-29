pub use super::text_field::TextField;
pub use super::widget::{Box, Widget};

use crate::user_interface::output::draw;

const BORDER_PADDING: u16 = 2 * draw::BORDER_SIZE;
const LABEL_HEIGHT: u16 = 1 + BORDER_PADDING;

pub struct Label {
    field: TextField,
}

impl Label {
    pub fn new(text: &str) -> Self {
        let width = Label::get_label_length(text);
        let height = LABEL_HEIGHT;
        let mut field = TextField::new(text, width, height);
        field.set_text(text);
        Self { field }
    }

    pub fn set_text(&mut self, text: &str) {
        self.field.set_text(text);
        self.field
            .set_size(Label::get_label_length(text), LABEL_HEIGHT);
    }

    fn get_label_length(text: &str) -> u16 {
        text.len() as u16 + BORDER_PADDING
    }
}

impl Widget for Label {
    fn raw_render(&mut self) {
        self.field.raw_render();
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.field.set_position(x, y);
    }

    fn set_size(&mut self, width: u16, height: u16) {
        self.field.set_size(width, height);
    }

    fn width(&self) -> u16 {
        self.field.width()
    }

    fn height(&self) -> u16 {
        self.field.height()
    }

    fn prev_geometry(&self) -> Box<u16> {
        self.field.prev_geometry()
    }
}
