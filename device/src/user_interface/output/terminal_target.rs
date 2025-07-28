use anyhow::{Context, Result};
use std::io::{Write, stdout};

use crossterm::{
    cursor::{Hide, MoveTo},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};

use embedded_graphics::{Pixel, draw_target::DrawTarget, pixelcolor::Rgb888, prelude::*};

pub const BORDER_SIZE: u16 = 1;

const WIDTH: usize = 120;
const HEIGHT: usize = 40;

#[derive(Clone)]
struct Frame {
    buffer: Vec<String>,
}

impl Frame {
    fn new() -> Self {
        Self {
            buffer: vec![" ".to_string(); HEIGHT * WIDTH],
        }
    }

    fn set(&mut self, x: u32, y: u32, ansi_str: &str) {
        let x = x as usize;
        let y = y as usize;
        if y < HEIGHT && x < WIDTH {
            self.buffer[y * WIDTH + x] = ansi_str.to_string();
        }
    }

    fn get(&self, x: u16, y: u16) -> Option<&str> {
        let x = x as usize;
        let y = y as usize;
        if y < HEIGHT && x < WIDTH {
            Some(&self.buffer[y * WIDTH + x])
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.buffer.fill(" ".to_string());
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

    pub fn flush(&mut self) -> Result<()> {
        let mut stdout = stdout();

        for y in 0..HEIGHT as u16 {
            for x in 0..WIDTH as u16 {
                let new = self.current_frame.get(x, y).unwrap();
                let old = self.previous_frame.get(x, y).unwrap();
                if new != old {
                    queue!(stdout, MoveTo(x, y), Print(new))
                        .with_context(|| format!("Failed to print at ({x}, {y})"))?;
                }
            }
        }

        stdout.flush().context("Failed to flush stdout")?;
        std::mem::swap(&mut self.current_frame, &mut self.previous_frame);
        self.current_frame.clear();

        Ok(())
    }
}

impl DrawTarget for Renderer {
    type Color = Rgb888;
    type Error = anyhow::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            if let Ok((x, y)) = coord.try_into() {
                let r = color.r();
                let g = color.g();
                let b = color.b();
                let ch = '█'; // можно заменить на другие символы
                let ansi = format!("\x1b[38;2;{r};{g};{b}m{ch}\x1b[0m");
                self.current_frame.set(x, y, &ansi);
            }
        }
        Ok(())
    }

    fn clear(&mut self, _color: Self::Color) -> Result<(), Self::Error> {
        execute!(stdout(), Clear(ClearType::All)).context("Cannot clear screen")?;
        self.current_frame.clear();
        self.previous_frame.clear();
        Ok(())
    }
}

impl OriginDimensions for Renderer {
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.clear(Rgb888::BLACK);
    }
}
