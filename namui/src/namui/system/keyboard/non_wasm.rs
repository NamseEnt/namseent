use super::KeyboardSystem;
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

impl KeyboardSystem {
    pub fn new() -> Self {
        let pressing_code_set = Arc::new(RwLock::new(HashSet::new()));

        KeyboardSystem { pressing_code_set }
    }
}
