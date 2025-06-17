use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use super::slot::{Slot, SlotRef};

pub type SlotId = usize;
pub type WeakSlotRef<Args> = Weak<RefCell<dyn Slot<Args>>>;

pub struct Signal<Args> {
    slots: HashMap<SlotId, WeakSlotRef<Args>>,
    next_id: SlotId,
}

impl<Args> Signal<Args> {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn connect(&mut self, slot: SlotRef<Args>) -> SlotId {
        let id = self.next_id;
        self.next_id += 1;
        self.slots.insert(id, Rc::downgrade(&slot));
        id
    }

    pub fn disconnect(&mut self, id: SlotId) -> bool {
        self.slots.remove(&id).is_some()
    }

    pub fn emit(&mut self, args: &Args) {
        for weak in self.slots.values() {
            if let Some(slot) = weak.upgrade() {
                slot.borrow_mut().call(args);
            }
        }
    }

    pub fn clean(&mut self) {
        self.slots.retain(|_, weak| weak.upgrade().is_some());
    }
}

#[macro_export]
macro_rules! define_signal_struct {
    ($(#[$meta:meta])* struct $name:ident, $args:ty) => {
        $(#[$meta])*
        pub struct $name {
            signal: Signal<$args>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    signal: Signal::new(),
                }
            }

            pub fn connect(&mut self, slot: SlotRef<$args>) -> SlotId {
                self.signal.connect(slot)
            }

            pub fn disconnect(&mut self, id: SlotId) -> bool {
                self.signal.disconnect(id)
            }

            pub fn emit(&mut self, args: &$args) {
                self.signal.emit(args);
            }

            pub fn clean(&mut self) {
                self.signal.clean();
            }
        }
    };
}
