use crate::{
    game_state::item::ItemKind,
    icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize},
    thumbnail::ThumbnailComposer,
};
use namui::*;

impl ItemKind {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            ItemKind::Heal { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Health)
                .build(),
            ItemKind::AttackPowerPlusBuff { .. } => Icon::new(IconKind::AttackDamage)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::AttackPowerMultiplyBuff { .. } => Icon::new(IconKind::AttackDamage)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::AttackSpeedPlusBuff { .. } => Icon::new(IconKind::AttackSpeed)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::AttackSpeedMultiplyBuff { .. } => Icon::new(IconKind::AttackSpeed)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::AttackRangePlus { .. } => Icon::new(IconKind::AttackRange)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::MovementSpeedDebuff { .. } => Icon::new(IconKind::MoveSpeed)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Down).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::RoundDamage { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
            ItemKind::RoundDamageOverTime { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
            ItemKind::Lottery { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            ItemKind::LinearDamage { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
            ItemKind::LinearDamageOverTime { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
            ItemKind::ExtraReroll => Icon::new(IconKind::Refresh)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::Shield { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Shield)
                .build(),
            ItemKind::DamageReduction { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
        }
    }
}
