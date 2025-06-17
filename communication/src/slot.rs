use mockall::*;

pub type SlotRef<Args> = std::rc::Rc<std::cell::RefCell<dyn Slot<Args>>>;

#[automock]
pub trait Slot<Args> {
    fn call(&mut self, args: &Args);
}

impl<Args, F> Slot<Args> for F
where
    F: FnMut(&Args),
{
    fn call(&mut self, args: &Args) {
        self(args)
    }
}

#[macro_export]
macro_rules! slot {
    ($name:ident, $args:ty, $body:block) => {
        let $name: $crate::SlotRef<$args> =
            std::rc::Rc::new(std::cell::RefCell::new(move |args: &$args| $body));
    };
}
