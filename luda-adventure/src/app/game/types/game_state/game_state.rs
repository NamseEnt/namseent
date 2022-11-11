use super::{CameraState, QuestState, TickState};

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
