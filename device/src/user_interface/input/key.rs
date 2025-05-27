use super::input_controller::{InputEvent, InterruptCapable};
use anyhow::{Context, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Key {
    id: u32,
    controller: Rc<RefCell<dyn InterruptCapable>>,
}

impl Key {
    pub fn new(id: u32, controller: Rc<RefCell<dyn InterruptCapable>>) -> Self {
        Self { id, controller }
    }

    #[allow(dead_code)]
    pub fn on_change<F>(&self, cb: F) -> Result<()>
    where
        F: Fn(InputEvent) + Send + 'static,
    {
        self.controller
            .borrow_mut()
            .register_callback(self.id, Box::new(cb))
            .context("Cannot register a key's callback")
    }

    #[allow(dead_code)]
    pub fn is_pressed(&self) -> bool {
        self.controller.borrow().read_line(self.id)
    }
}
