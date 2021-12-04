use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;

use super::Xy;

pub struct EngineState {
    pub mouse_position: Xy<i16>,
}

lazy_static! {
    static ref ENGINE_STATE: RwLock<Arc<EngineState>> = RwLock::new(Arc::new(EngineState {
        mouse_position: Xy { x: 0, y: 0 },
    }));
}

pub fn get_engine_state() -> Arc<EngineState> {
    ENGINE_STATE.read().unwrap().clone()
}

pub fn update_engine_state(state: EngineState) {
    *ENGINE_STATE.write().unwrap() = Arc::new(state);
}
