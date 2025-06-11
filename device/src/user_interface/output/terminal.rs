use anyhow::{Context, Result};
use std::io::{Write, stdout};

use crossterm::{
    cursor::{Hide, MoveTo},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};

pub use super::output_renderer::OutputRenderer;
pub const BORDER_SIZE: u16 = 1;

const WIDTH: usize = 120;
const HEIGHT: usize = 40;

#[derive(Clone)]
struct Frame {
    buffer: Vec<char>,
}

impl Frame {
    fn new() -> Self {
        Self {
            buffer: vec![' '; HEIGHT * WIDTH],
        }
    }

    fn set(&mut self, x: u16, y: u16, text: &str) {
        let x = x as usize;
        let y = y as usize;
        if y >= HEIGHT {
            return;
        }

        for (i, ch) in text.chars().enumerate() {
            if x + i < WIDTH {
                self.buffer[y * WIDTH + x + i] = ch;
            } else {
                break;
            }
        }
    }

    fn get(&self, x: u16, y: u16) -> Option<char> {
        let x = x as usize;
        let y = y as usize;
        if y < HEIGHT && x < WIDTH {
            Some(self.buffer[y * WIDTH + x])
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.buffer.fill(' ');
    }
}

pub struct Renderer {
    current_frame: Frame,
    previous_frame: Frame,
}

impl Renderer {
    pub fn new() -> Self {
        let mut stdout = stdout();
        execute!(stdout, Hide, Clear(ClearType::All)).unwrap();
        Self {
            current_frame: Frame::new(),
            previous_frame: Frame::new(),
        }
    }
}

impl OutputRenderer for Renderer {
    fn draw_text(&mut self, x: u16, y: u16, text: &str) -> Result<()> {
        self.current_frame.set(x, y, text);
        Ok(())
    }

    fn draw_box(&mut self, x: u16, y: u16, width: u16, height: u16) -> Result<()> {
        if width < 2 || height < 2 {
            return Ok(());
        }

        let top = format!("┌{:─<width$}┐", "", width = (width - 2) as usize);
        let bottom = format!("└{:─<width$}┘", "", width = (width - 2) as usize);
        self.draw_text(x, y, &top)?;
        self.draw_text(x, y + height - 1, &bottom)?;

        for dy in 1..(height - 1) {
            self.draw_text(x, y + dy, "│")?;
            self.draw_text(x + width - 1, y + dy, "│")?;
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        let mut stdout = stdout();

        for y in 0..HEIGHT as u16 {
            for x in 0..WIDTH as u16 {
                let new = self.current_frame.get(x, y).unwrap();
                let old = self.previous_frame.get(x, y).unwrap();
                if new != old {
                    queue!(stdout, MoveTo(x, y), Print(new))
                        .with_context(|| format!("Failed to print char '{new}' at ({x}, {y})"))?;
                }
            }
        }

        stdout.flush().context("Failed to flush stdout")?;
        std::mem::swap(&mut self.current_frame, &mut self.previous_frame);
        self.current_frame.clear();

        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All)).context("Cannot clear screen")?;
        self.current_frame.clear();
        self.previous_frame.clear();
        Ok(())
    }

    fn clear_box(&mut self, x: u16, y: u16, width: u16, height: u16) -> Result<()> {
        let blank = " ".repeat(width as usize);
        for dy in 0..height {
            self.current_frame.set(x, y + dy, &blank);
        }
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.clear();
    }
}
