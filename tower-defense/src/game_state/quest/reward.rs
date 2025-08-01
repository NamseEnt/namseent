use crate::{
    game_state::{
        GameState,
        item::{Item, generate_item},
        upgrade::{Upgrade, generate_upgrade},
    },
    rarity::Rarity,
};
use rand::{Rng, seq::SliceRandom, thread_rng};

#[derive(Debug, Clone)]
pub enum QuestReward {
    Money { amount: usize },
    Item { item: Item },
    Upgrade { upgrade: Upgrade },
}
impl QuestReward {
    pub fn description(&self, game_state: &GameState) -> String {
        use crate::l10n::quest::QuestRewardText;
        let text_manager = &game_state.text();
        match self {
            Self::Money { amount } => {
                text_manager.quest_reward(QuestRewardText::Money { amount: *amount })
            }
            Self::Item { item } => format!(
                "{}: {}",
                text_manager.quest_reward(QuestRewardText::Item),
                item.kind.description(text_manager)
            ),
            Self::Upgrade { upgrade } => format!(
                "{}: {}",
                text_manager.quest_reward(QuestRewardText::Upgrade),
                upgrade.kind.description(text_manager)
            ),
        }
    }
}
pub(super) fn generate_quest_reward(game_state: &GameState, rarity: Rarity) -> QuestReward {
    match [(0, 0.2), (1, 0.1), (2, 0.7)]
        .choose_weighted(&mut thread_rng(), |x| x.1)
        .unwrap()
        .0
    {
        0 => QuestReward::Money {
            amount: thread_rng().gen_range(match rarity {
                Rarity::Common => 10..25,
                Rarity::Rare => 25..50,
                Rarity::Epic => 50..100,
                Rarity::Legendary => 100..500,
            }),
        },
        1 => QuestReward::Item {
            item: generate_item(rarity),
        },
        2 => QuestReward::Upgrade {
            upgrade: generate_upgrade(game_state, rarity),
        },
        _ => panic!("Invalid QuestReward"),
    }
}
