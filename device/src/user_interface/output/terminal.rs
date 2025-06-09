use anyhow::{Context, Result};
use std::collections::HashMap;
use std::io::{Write, stdout};

pub use super::output_renderer::OutputRenderer;

use crossterm::{
    cursor::Hide,
    cursor::MoveTo,
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};

type Pos = (u16, u16);

#[derive(Default)]
struct Frame {
    cells: HashMap<Pos, String>,
}

impl Frame {
    pub fn set(&mut self, x: u16, y: u16, text: &str) {
        self.cells.insert((x, y), text.to_string());
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&String> {
        self.cells.get(&(x, y))
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }
}

pub struct Renderer {
    current_frame: Frame,
    previous_frame: Frame,
}

impl Renderer {
    pub fn new() -> Self {
        let r = Self {
            current_frame: Frame::default(),
            previous_frame: Frame::default(),
        };
        execute!(stdout(), Hide, Clear(ClearType::All)).unwrap();
        r
    }
}

impl OutputRenderer for Renderer {
    fn draw_text(&mut self, x: u16, y: u16, text: &str) -> Result<()> {
        self.current_frame.set(x, y, text);
        Ok(())
    }

    fn draw_box(&mut self, x: u16, y: u16, width: u16, height: u16) -> Result<()> {
        let horizontal = "─".repeat((width - 2) as usize);
        let top = format!("┌{}┐", horizontal);
        let bottom = format!("└{}┘", horizontal);

        let _ = self.draw_text(x, y, &top);
        let _ = self.draw_text(x, y + height - 1, &bottom);

        for j in 1..(height - 1) {
            let _ = self.draw_text(x, y + j, "│");
            let _ = self.draw_text(x + width - 1, y + j, "│");
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        let mut stdout = stdout();

        for ((x, y), text) in &self.current_frame.cells {
            match self.previous_frame.get(*x, *y) {
                Some(prev) if prev == text => continue,
                _ => {
                    queue!(stdout, MoveTo(*x, *y), Print(text)).with_context(|| {
                        format!("Failed to print text \"{text}\" on a position ({x}, {y})")
                    })?;
                }
            }
        }

        stdout.flush().context("Cannot flash buffer")?;
        self.previous_frame = std::mem::take(&mut self.current_frame);
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All)).context("Cannot clear the screen")?;
        self.current_frame.clear();
        self.previous_frame.clear();
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.clear();
    }
}
