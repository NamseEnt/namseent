use crate::{
    game_state::effect::Effect,
    icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize},
    thumbnail::ThumbnailComposer,
};
use namui::*;

impl Effect {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            Effect::Heal { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Health)
                .build(),
            Effect::Lottery { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::ExtraReroll => Icon::new(IconKind::Refresh)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            Effect::ExtraShopReroll => Icon::new(IconKind::Refresh)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            Effect::Shield { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Shield)
                .build(),
            Effect::EarnGold { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::DamageReduction { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Damage)
                .build(),
            Effect::UserDamageReduction { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Damage)
                .build(),
            Effect::LoseHealth { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Health)
                .build(),
            Effect::LoseHealthRange { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Health)
                .build(),
            Effect::LoseGoldRange { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::LoseHealthExpire { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Health)
                .build(),
            Effect::LoseGoldExpire { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::LoseGold { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::GrantUpgrade { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Refresh)
                .build(),
            Effect::GrantItem { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Item)
                .build(),
            Effect::AddChallengeMonster => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Damage)
                .build(),
            Effect::IncreaseAllTowersDamage { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Damage)
                .build(),
            Effect::DecreaseAllTowersDamage { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Damage)
                .build(),
            Effect::DecreaseIncomingDamage { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Damage)
                .build(),
            Effect::IncreaseCardSelectionHandMaxSlots { .. } => {
                ThumbnailComposer::new(width_height)
                    .with_icon_base(IconKind::Card)
                    .build()
            }
            Effect::IncreaseCardSelectionHandMaxRerolls { .. } => {
                ThumbnailComposer::new(width_height)
                    .with_icon_base(IconKind::Refresh)
                    .build()
            }
            Effect::IncreaseShopMaxRerolls { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Refresh)
                .build(),
            Effect::IncreaseGoldGain { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::DecreaseGoldGainPercent { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::IncreaseIncomingDamage { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Damage)
                .build(),
            Effect::DisableItemAndUpgradePurchases => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Item)
                .build(),
            Effect::DisableItemUse => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Reject)
                .build(),
            Effect::DecreaseCardSelectionHandMaxSlots { .. } => {
                ThumbnailComposer::new(width_height)
                    .with_icon_base(IconKind::Card)
                    .build()
            }
            Effect::DecreaseCardSelectionHandMaxRerolls { .. } => {
                ThumbnailComposer::new(width_height)
                    .with_icon_base(IconKind::Refresh)
                    .build()
            }
            Effect::DecreaseShopMaxRerolls { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Shop)
                .build(),
            Effect::AddCardSelectionHandRerollHealthCost { .. } => {
                ThumbnailComposer::new(width_height)
                    .with_icon_base(IconKind::Health)
                    .build()
            }
            Effect::AddShopRerollHealthCost { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Health)
                .build(),
            Effect::DecreaseEnemyHealthPercent { .. } => Icon::new(IconKind::Health)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            Effect::RankTowerDisable { .. } => Icon::new(IconKind::Damage)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Reject).position(IconAttributePosition::Center),
                ])
                .to_rendering_tree(),
            Effect::SuitTowerDisable { .. } => Icon::new(IconKind::Card)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Reject).position(IconAttributePosition::Center),
                ])
                .to_rendering_tree(),
            Effect::AddTowerCardToPlacementHand {
                tower_kind,
                suit,
                rank,
                ..
            } => match tower_kind {
                crate::TowerKind::Barricade => ThumbnailComposer::new(width_height)
                    .with_tower_image(*tower_kind)
                    .build(),
                _ => ThumbnailComposer::new(width_height)
                    .with_tower_image(*tower_kind)
                    .add_rank_overlay(*rank)
                    .add_suit_overlay(*suit)
                    .build(),
            },
            Effect::GainShield { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Shield)
                .build(),
            Effect::HealHealth { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Health)
                .build(),
            Effect::GainGold { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
        }
    }
}
