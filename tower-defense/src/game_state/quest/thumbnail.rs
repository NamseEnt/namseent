use crate::{
    game_state::quest::QuestRequirement,
    icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize},
    thumbnail::ThumbnailComposer,
};
use namui::*;

impl QuestRequirement {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            QuestRequirement::BuildTowerRankNew { rank, count } => {
                ThumbnailComposer::new(width_height)
                    .with_default_tower()
                    .add_count_overlay(*count)
                    .add_new_indicator()
                    .add_rank_overlay(*rank)
                    .build()
            }
            QuestRequirement::BuildTowerRank { rank, count } => {
                ThumbnailComposer::new(width_height)
                    .with_default_tower()
                    .add_count_overlay(*count)
                    .add_rank_overlay(*rank)
                    .build()
            }
            QuestRequirement::BuildTowerSuitNew { suit, count } => {
                ThumbnailComposer::new(width_height)
                    .with_default_tower()
                    .add_count_overlay(*count)
                    .add_new_indicator()
                    .add_suit_overlay(*suit)
                    .build()
            }
            QuestRequirement::BuildTowerSuit { suit, count } => {
                ThumbnailComposer::new(width_height)
                    .with_default_tower()
                    .add_count_overlay(*count)
                    .add_suit_overlay(*suit)
                    .build()
            }
            QuestRequirement::BuildTowerHandNew { hand, count } => {
                ThumbnailComposer::new(width_height)
                    .with_tower_image(*hand)
                    .add_count_overlay(*count)
                    .add_new_indicator()
                    .build()
            }
            QuestRequirement::BuildTowerHand { hand, count } => {
                ThumbnailComposer::new(width_height)
                    .add_count_overlay(*count)
                    .with_tower_image(*hand)
                    .build()
            }
            QuestRequirement::ClearBossRoundWithoutItems => Icon::new(IconKind::EnemyBoss)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Item).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            QuestRequirement::DealDamageWithItems { .. } => Icon::new(IconKind::AttackDamage)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Item).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            QuestRequirement::BuildTowersWithoutReroll { count } => {
                ThumbnailComposer::new(width_height)
                    .with_default_tower()
                    .add_count_overlay(*count)
                    .add_no_reroll_indicator()
                    .build()
            }
            QuestRequirement::UseReroll { count } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Refresh)
                .add_count_overlay(*count)
                .build(),
            QuestRequirement::SpendGold { .. } => Icon::new(IconKind::Gold)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Down).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            QuestRequirement::EarnGold { .. } => Icon::new(IconKind::Gold)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
        }
    }
}
