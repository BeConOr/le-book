pub use super::widget;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Layout: widget::Widget {
    fn add_widget(&mut self, widget: Rc<RefCell<dyn widget::Widget>>);
}
