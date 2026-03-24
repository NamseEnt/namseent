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
            Effect::ExtraDice => Icon::new(IconKind::Refresh)
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

            Effect::LoseGold { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::GrantUpgrade { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Refresh)
                .build(),
            Effect::GrantItem { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Item)
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
            Effect::IncreaseMaxHandSlots { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Card)
                .build(),
            Effect::IncreaseMaxRerolls { .. } => ThumbnailComposer::new(width_height)
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
            Effect::DecreaseMaxHandSlots { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Card)
                .build(),
            Effect::DecreaseMaxRerolls { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Refresh)
                .build(),
            Effect::IncreaseEnemyHealthPercent { .. } => Icon::new(IconKind::Health)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            Effect::DecreaseEnemyHealthPercent { .. } => Icon::new(IconKind::Health)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Down).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            Effect::IncreaseEnemySpeed { .. } => Icon::new(IconKind::MoveSpeed)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            Effect::DecreaseEnemySpeed { .. } => Icon::new(IconKind::MoveSpeed)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Down).position(IconAttributePosition::BottomRight),
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
