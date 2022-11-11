use super::Quest;
use crate::app::game::known_id;
use namui::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct QuestState {
    quest_progress_map: HashMap<Uuid, usize>,
}

impl QuestState {
    pub fn new() -> Self {
        let mut quest_progress_map = HashMap::new();
        quest_progress_map.insert(known_id::quest::FIRST_QUEST, 0);
        Self { quest_progress_map }
    }

    pub fn get_quest_object_list<'a>(
        &self,
        ecs_app: &'a crate::ecs::App,
    ) -> Vec<&'a crate::ecs::Entity> {
        let quest_object_id_set = self
            .quest_progress_map
            .iter()
            .flat_map(|(quest_id, action_index)| {
                let quest = Quest::get_quest(*quest_id);
                quest.get_quest_object_list(*action_index)
            })
            .collect::<HashSet<_>>();
        ecs_app
            .entities()
            .filter(|entity| quest_object_id_set.contains(&entity.id()))
            .collect()
    }
}
