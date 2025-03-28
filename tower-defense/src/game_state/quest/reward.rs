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
    pub fn description(&self) -> String {
        match self {
            Self::Money { amount } => format!("${} 골드", amount),
            Self::Item { item } => format!("Item: {}", item.kind.description()),
            Self::Upgrade { upgrade } => format!("Upgrade: {}", upgrade.kind.description()),
        }
    }
}
pub(super) fn generate_quest_reward(game_state: &GameState, rarity: Rarity) -> QuestReward {
    match [(0, 0.2), (1, 0.3), (2, 0.5)]
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
