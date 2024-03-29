use super::QuestAction;
use crate::app::game::known_id;
use namui::*;

pub struct Quest {
    pub id: Uuid,
    pub action_list: Vec<QuestAction>,
}

impl Quest {
    pub fn get_quest(_id: Uuid) -> Self {
        mock_quest()
    }

    pub fn get_quest_object_list(&self, action_index: usize) -> Vec<Uuid> {
        match self.action_list.get(action_index) {
            Some(action) => match action {
                QuestAction::WaitForUserInteractObject(id) => vec![*id],
            },
            None => Vec::new(),
        }
    }
}

fn mock_quest() -> Quest {
    Quest {
        id: known_id::quest::FIRST_QUEST,
        action_list: vec![QuestAction::WaitForUserInteractObject(
            known_id::object::FIRST_QUEST_OBJECT,
        )],
    }
}
