use crate::{game_state::GameState, rarity::Rarity};
use rand::{Rng, thread_rng};

#[derive(Debug, Clone)]
pub enum QuestReward {
    Money { amount: usize },
}
impl QuestReward {
    pub fn description(&self, game_state: &GameState) -> String {
        use crate::l10n::quest::QuestRewardText;
        let text_manager = &game_state.text();
        match self {
            Self::Money { amount } => {
                text_manager.quest_reward(QuestRewardText::Money { amount: *amount })
            }
        }
    }
}
pub(super) fn generate_quest_reward(_game_state: &GameState, rarity: Rarity) -> QuestReward {
    QuestReward::Money {
        amount: thread_rng().gen_range(match rarity {
            Rarity::Common => 25..50,
            Rarity::Rare => 50..100,
            Rarity::Epic => 100..250,
            Rarity::Legendary => 250..500,
        }),
    }
}
