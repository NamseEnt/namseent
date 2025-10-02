mod wasm;

use crate::*;
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};
pub(crate) use wasm::*;

struct KeyboardSystem {
    pressing_code_set: Arc<RwLock<HashSet<Code>>>,
}

impl KeyboardSystem {
    pub(crate) fn new() -> Self {
        let pressing_code_set = Arc::new(RwLock::new(HashSet::new()));

        KeyboardSystem { pressing_code_set }
    }
}

lazy_static::lazy_static! {
    static ref KEYBOARD_SYSTEM: Arc<KeyboardSystem> = Arc::new(KeyboardSystem::new());
}

pub(super) async fn init() -> Result<()> {
    lazy_static::initialize(&KEYBOARD_SYSTEM);
    Ok(())
}

pub fn any_code_press(codes: impl IntoIterator<Item = Code>) -> bool {
    let pressing_code_set = KEYBOARD_SYSTEM.pressing_code_set.read().unwrap();
    for code in codes {
        if pressing_code_set.contains(&code) {
            return true;
        }
    }
    false
}

fn record_key_down(code: Code) {
    let mut pressing_code_set = KEYBOARD_SYSTEM.pressing_code_set.write().unwrap();
    pressing_code_set.insert(code);
}

fn record_key_up(code: Code) {
    let mut pressing_code_set = KEYBOARD_SYSTEM.pressing_code_set.write().unwrap();
    pressing_code_set.remove(&code);
}

fn pressing_code_set() -> HashSet<Code> {
    KEYBOARD_SYSTEM.pressing_code_set.read().unwrap().clone()
}

fn clear_pressing_code_set() {
    KEYBOARD_SYSTEM.pressing_code_set.write().unwrap().clear()
}

pub fn shift_press() -> bool {
    any_code_press([Code::ShiftLeft, Code::ShiftRight])
}
pub fn ctrl_press() -> bool {
    any_code_press([Code::ControlLeft, Code::ControlRight])
}
pub fn alt_press() -> bool {
    any_code_press([Code::AltLeft, Code::AltRight])
}
