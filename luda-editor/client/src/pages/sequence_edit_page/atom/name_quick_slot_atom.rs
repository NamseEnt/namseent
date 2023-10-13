use namui::prelude::*;

pub static NAME_QUICK_SLOT: Atom<NameQuickSlot> = Atom::uninitialized_new();

#[derive(Debug)]
pub struct NameQuickSlot {
    pub names: [String; 5],
}
impl NameQuickSlot {
    pub fn new() -> Self {
        Self {
            names: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
        }
    }
}
