use anyhow::Result;

#[allow(dead_code)]
pub trait OutputRenderer {
    fn draw_text(&mut self, x: u16, y: u16, text: &str) -> Result<()>;
    fn draw_box(&mut self, x: u16, y: u16, width: u16, height: u16) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
    fn clear(&mut self) -> Result<()>;
    fn clear_box(&mut self, x: u16, y: u16, width: u16, height: u16) -> Result<()>;
}
