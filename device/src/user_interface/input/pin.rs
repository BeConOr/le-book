use input_controller::{InputEvent, InterruptCapable};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Pin {
    id: u32,
    controller: Rc<RefCell<dyn InterruptCapable>>,
}

impl Pin {
    pub fn new(id: u32, controller: Rc<RefCell<dyn InterruptCapable>>) -> Self {
        Self { id, controller }
    }

    pub fn on_change<F>(&self, cb: F)
    where
        F: Fn(InputEvent) + Send + 'static,
    {
        self.controller
            .borrow_mut()
            .register_callback(self.id, Box::new(cb));
    }

    pub fn is_high(&self) -> bool {
        self.controller.borrow().read_line(self.id)
    }
}
