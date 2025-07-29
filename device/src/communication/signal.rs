// signal_slot.rs
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use super::slot::{Slot, SlotRef};

pub type SlotId = usize;
pub type WeakSlotRef<'a, Args> = Weak<RefCell<dyn Slot<'a, Args> + 'a>>;

pub struct Signal<'a, Args: 'a> {
    slots: HashMap<SlotId, WeakSlotRef<'a, Args>>,
    next_id: SlotId,
}

impl<'a, Args> Signal<'a, Args> {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn connect(&mut self, slot: &SlotRef<'a, Args>) -> SlotId {
        let id = self.next_id;
        self.next_id += 1;
        self.slots.insert(id, Rc::downgrade(slot));
        id
    }

    pub fn disconnect(&mut self, id: SlotId) -> bool {
        self.slots.remove(&id).is_some()
    }

    pub fn emit(&mut self, args: &'a Args) {
        self.slots.retain(|_, weak| weak.upgrade().is_some());
        for weak in self.slots.values() {
            if let Some(slot) = weak.upgrade() {
                slot.borrow_mut().call(args);
            }
        }
    }
}

#[macro_export]
macro_rules! define_signal_struct {
    ($(#[$meta:meta])* struct $name:ident, $args:ty) => {
        $(#[$meta])*
        pub struct $name<'a> {
            signal: $crate::signal::Signal<'a, $args>,
        }

        impl<'a> $name<'a> {
            pub fn new() -> Self {
                Self {
                    signal: $crate::signal::Signal::new(),
                }
            }

            pub fn connect(&mut self, slot: &$crate::SlotRef<'a, $args>) -> $crate::signal::SlotId {
                self.signal.connect(slot)
            }

            pub fn disconnect(&mut self, id: $crate::signal::SlotId) -> bool {
                self.signal.disconnect(id)
            }

            pub fn emit(&mut self, args: &'a $args) {
                self.signal.emit(args);
            }
        }
    };
}

#[macro_export]
macro_rules! signal {
    ($vis:vis $field:ident : $args:ty) => {
        $vis $field: $crate::Signal<'a, $args>
    };
}

#[macro_export]
macro_rules! init_signal {
    ($self:ident.$field:ident) => {
        $self.$field = $crate::Signal::new();
    };
}

#[macro_export]
macro_rules! emit_signal {
    ($self:ident.$field:ident, $args:expr) => {
        $self.$field.emit($args);
    };
}

#[macro_export]
macro_rules! connect_signal {
    ($self:ident.$field:ident, $slot:expr) => {
        $self.$field.connect($slot)
    };
}

#[macro_export]
macro_rules! disconnect_signal {
    ($self:ident.$field:ident, $id:expr) => {
        $self.$field.disconnect($id)
    };
}
