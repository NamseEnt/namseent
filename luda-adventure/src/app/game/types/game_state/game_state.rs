use super::{CharacterState, QuestState};

pub struct GameState {
    pub character: CharacterState,
    pub quest: QuestState,
}
impl GameState {
    pub fn new() -> Self {
        Self {
            character: CharacterState::new(),
            quest: QuestState::new(),
        }
    }
}
