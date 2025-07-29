pub use super::widget;

use crate::user_interface::output::draw;

const BORDER_PADDING: u16 = 2 * draw::BORDER_SIZE;

pub struct TextField {
    text: String,
    cur_box: widget::Box<u16>,
    prev_box: widget::Box<u16>,
}

impl TextField {
    pub fn new(text: &str, width: u16, height: u16) -> Self {
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
    }

    fn render_lines(&self) -> Vec<String> {
        let inner_width = self.cur_box.width.saturating_sub(BORDER_PADDING) as usize;
        let inner_height = self.cur_box.height.saturating_sub(BORDER_PADDING);

        if inner_width == 0 || inner_height == 0 {
            return vec![];
        }

        let mut lines = vec![];
        let mut current_line = String::new();
        let mut current_len = 0;

        for word in self.text.split_whitespace() {
            let word_len = word.len();

            if current_len != 0 && current_len + 1 + word_len > inner_width {
                lines.push(current_line);
                current_line = String::new();
                current_len = 0;

                if lines.len() as u16 == inner_height {
                    break;
                }
            }

            if !current_line.is_empty() {
                current_line.push(' ');
                current_len += 1;
            }

            current_line.push_str(word);
            current_len += word_len;
        }

        if !current_line.is_empty() && (lines.len() as u16) < inner_height {
            lines.push(current_line);
        }

        lines
    }
}

impl widget::Widget for TextField {
    fn raw_render(&mut self) {
        self.prev_box = self.cur_box;

        let start_x = self.cur_box.x + draw::BORDER_SIZE;
        let start_y = self.cur_box.y + draw::BORDER_SIZE;

        for (i, line) in self.render_lines().into_iter().enumerate() {
            draw::draw_text(start_x, start_y + i as u16, &line);
        }

        draw::draw_box(
            self.cur_box.x,
            self.cur_box.y,
            self.cur_box.width,
            self.cur_box.height,
        );
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.cur_box.x = x;
        self.cur_box.y = y;
    }

    fn set_size(&mut self, width: u16, height: u16) {
        self.cur_box.width = width;
        self.cur_box.height = height;
    }

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
