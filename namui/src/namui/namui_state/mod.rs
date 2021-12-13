use super::Xy;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

pub struct NamuiState {
    pub mouse_position: Xy<i16>,
}

lazy_static! {
    static ref NAMUI_STATE: RwLock<Arc<NamuiState>> = RwLock::new(Arc::new(NamuiState {
        mouse_position: Xy { x: 0, y: 0 },
    }));
}

pub fn get_namui_state() -> Arc<NamuiState> {
    NAMUI_STATE.read().unwrap().clone()
}

pub fn update_namui_state(state: NamuiState) {
    *NAMUI_STATE.write().unwrap() = Arc::new(state);
}
