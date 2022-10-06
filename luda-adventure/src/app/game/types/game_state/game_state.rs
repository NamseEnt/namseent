use super::QuestState;

pub struct GameState {
    pub quest: QuestState,
}
impl GameState {
    pub fn new() -> Self {
        Self {
            quest: QuestState::new(),
        }
    }
}
