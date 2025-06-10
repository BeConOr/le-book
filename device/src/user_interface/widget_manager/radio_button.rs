use super::widget::{Box, Widget};
use crate::user_interface::output::draw;

pub struct RadioButton {
    pub cur_box: Box<u16>,
    pub prev_box: Box<u16>,
    pub label: String,
    pub selected: bool,
}

impl RadioButton {
    pub fn new(label: &str) -> Self {
        let cur_box = Box::<u16> {
            x: 0,
            y: 0,
            width: RadioButton::generate_view(" ", label).len() as u16,
            height: 1,
        };
        Self {
            cur_box,
            prev_box: cur_box,
            label: label.to_string(),
            selected: false,
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_text(&mut self, label: &str) {
        self.label = label.to_string();
        self.cur_box.width = RadioButton::generate_view(" ", label).len() as u16;
    }

    fn generate_view(symbol: &str, label: &str) -> String {
        format!("({}) {}", symbol, label)
    }
}

impl Widget for RadioButton {
    fn raw_render(&mut self) {
        let symbol = if self.selected { "●" } else { " " };
        let text: String = RadioButton::generate_view(symbol, &self.label);

        draw::draw_text(self.cur_box.x, self.cur_box.y, &text);
        self.prev_box = self.cur_box;
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

    fn prev_geometry(&self) -> Box<u16> {
        self.prev_box
    }
}
