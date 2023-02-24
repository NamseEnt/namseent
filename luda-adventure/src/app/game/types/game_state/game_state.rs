use super::{CameraState, QuestState, TickState};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub quest: QuestState,
    pub tick: TickState,
    pub camera: CameraState,
}
impl GameState {
    pub fn new() -> Self {
        Self {
            quest: QuestState::new(),
            tick: TickState::new(),
            camera: CameraState::new(),
        }
    }
}
