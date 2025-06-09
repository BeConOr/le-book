pub trait Widget {
    fn render(&mut self);
    fn set_position(&mut self, x: u16, y: u16);
    fn set_size(&mut self, width: u16, height: u16);
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}
