pub type SlotRef<'a, Args> = std::rc::Rc<std::cell::RefCell<dyn Slot<'a, Args> + 'a>>;

pub trait Slot<'a, Args: 'a> {
    fn call(&mut self, args: &'a Args);
}

impl<'a, Args: 'a, F> Slot<'a, Args> for F
where
    F: FnMut(&'a Args),
{
    fn call(&mut self, args: &'a Args) {
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
