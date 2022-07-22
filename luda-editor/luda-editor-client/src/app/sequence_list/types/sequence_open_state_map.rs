use std::collections::HashMap;

pub struct SequenceOpenStateMap {
    map: HashMap<String, SequenceOpenState>,
}
impl SequenceOpenStateMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn get(&self, title: &str) -> &SequenceOpenState {
        self.map.get(title).unwrap_or(&SequenceOpenState::Idle)
    }
    pub fn set(&mut self, title: String, state: SequenceOpenState) {
        self.map.insert(title, state);
    }
}

#[derive(Clone)]
pub enum SequenceOpenState {
    Idle,
    Opening,
    Failed { message: String },
}
