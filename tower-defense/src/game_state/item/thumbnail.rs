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
            Effect::Shield { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Shield)
                .build(),
            Effect::EarnGold { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::DamageReduction { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
            Effect::UserDamageReduction { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
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
            Effect::AddChallengeMonster => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
            Effect::IncreaseAllTowersDamage { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
            Effect::DecreaseAllTowersDamage { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
            Effect::IncreaseAllTowersAttackSpeed { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackSpeed)
                .build(),
            Effect::IncreaseAllTowersRange { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackRange)
                .build(),
            Effect::DecreaseIncomingDamage { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
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
            Effect::DecreaseGoldGainPercentDuringContract { .. } => {
                ThumbnailComposer::new(width_height)
                    .with_icon_base(IconKind::Gold)
                    .build()
            }
            Effect::IncreaseIncomingDamage { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
            Effect::DisableItemAndUpgradePurchasesDuringContract => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Item)
                .build(),
            Effect::DecreaseCardSelectionHandMaxSlots { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Card)
                .build(),
        }
    }
}
