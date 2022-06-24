use crate::Code;
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

pub struct KeyboardManager {
    pressing_code_set: Arc<RwLock<HashSet<Code>>>,
}

impl KeyboardManager {
    pub fn any_code_press(&self, codes: impl IntoIterator<Item = Code>) -> bool {
        let pressing_code_set = self.pressing_code_set.read().unwrap();
        for code in codes {
            if pressing_code_set.contains(&code) {
                return true;
            }
        }
        false
    }
    pub fn new() -> Self {
        let pressing_code_set = Arc::new(RwLock::new(HashSet::new()));

        KeyboardManager {
            pressing_code_set: pressing_code_set.clone(),
        }
    }
}
