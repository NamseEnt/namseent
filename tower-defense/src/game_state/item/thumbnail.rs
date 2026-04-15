use crate::{
    game_state::effect::Effect,
    icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize},
    thumbnail::{render_card_thumbnail, render_sticker_image, STICKER_THUMBNAIL_STROKE, ThumbnailComposer},
};
use namui::*;

impl Effect {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            Effect::Heal { .. } => render_sticker_image(IconKind::Health.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::Lottery { .. } => render_sticker_image(IconKind::Gold.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::ExtraDice => Icon::new(IconKind::Refresh)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            Effect::Shield { .. } => render_sticker_image(IconKind::Shield.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::EarnGold { .. } => render_sticker_image(IconKind::Gold.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::DamageReduction { .. } => render_sticker_image(IconKind::Damage.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::UserDamageReduction { .. } => render_sticker_image(IconKind::Damage.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::LoseHealth { .. } => render_sticker_image(IconKind::Health.image(), width_height, STICKER_THUMBNAIL_STROKE),

            Effect::LoseGold { .. } => render_sticker_image(IconKind::Gold.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::GrantUpgrade { .. } => render_sticker_image(IconKind::Refresh.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::GrantItem { .. } => render_sticker_image(IconKind::Item.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::IncreaseAllTowersDamage { .. } => render_sticker_image(IconKind::Damage.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::DecreaseAllTowersDamage { .. } => render_sticker_image(IconKind::Damage.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::DecreaseIncomingDamage { .. } => render_sticker_image(IconKind::Damage.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::IncreaseMaxHandSlots { .. } => render_sticker_image(IconKind::Card.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::IncreaseMaxRerolls { .. } => render_sticker_image(IconKind::Refresh.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::IncreaseGoldGain { .. } => render_sticker_image(IconKind::Gold.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::DecreaseGoldGainPercent { .. } => render_sticker_image(IconKind::Gold.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::IncreaseIncomingDamage { .. } => render_sticker_image(IconKind::Damage.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::DisableItemAndUpgradePurchases => render_sticker_image(IconKind::Item.image(), width_height, STICKER_THUMBNAIL_STROKE),
            Effect::DisableItemUse => render_sticker_image(IconKind::Reject.image(), width_height, STICKER_THUMBNAIL_STROKE),
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
            Effect::AddCardToHand { card } => render_card_thumbnail(card, width_height, STICKER_THUMBNAIL_STROKE),
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
