use super::{QuestState, TickState};

pub struct GameState {
    pub quest: QuestState,
    pub tick: TickState,
}
impl GameState {
    pub fn new() -> Self {
        Self {
            quest: QuestState::new(),
            tick: TickState::new(),
        }
    }
}
