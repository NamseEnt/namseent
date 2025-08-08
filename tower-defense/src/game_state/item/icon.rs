use crate::{
    game_state::item::ItemKind,
    icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize},
};
use namui::*;

impl ItemKind {
    pub fn icon(&self, wh: Wh<Px>) -> RenderingTree {
        match self {
            ItemKind::Heal { .. } => namui::render([Icon::new(IconKind::Health)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .to_rendering_tree()]),
            ItemKind::AttackPowerPlusBuff { .. } => Icon::new(IconKind::AttackDamage)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::AttackPowerMultiplyBuff { .. } => Icon::new(IconKind::AttackDamage)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::AttackSpeedPlusBuff { .. } => Icon::new(IconKind::AttackSpeed)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::AttackSpeedMultiplyBuff { .. } => Icon::new(IconKind::AttackSpeed)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::AttackRangePlus { .. } => Icon::new(IconKind::AttackRange)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::MovementSpeedDebuff { .. } => Icon::new(IconKind::MoveSpeed)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Down).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::RoundDamage { .. } => Icon::new(IconKind::AttackDamage)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .to_rendering_tree(),
            ItemKind::RoundDamageOverTime { .. } => Icon::new(IconKind::AttackDamage)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .to_rendering_tree(),
            ItemKind::Lottery { .. } => Icon::new(IconKind::Gold)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .to_rendering_tree(),
            ItemKind::LinearDamage { .. } => Icon::new(IconKind::AttackDamage)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .to_rendering_tree(),
            ItemKind::LinearDamageOverTime { .. } => Icon::new(IconKind::AttackDamage)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .to_rendering_tree(),
            ItemKind::ExtraReroll => Icon::new(IconKind::Refresh)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            ItemKind::Shield { .. } => Icon::new(IconKind::Shield)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .to_rendering_tree(),
            ItemKind::DamageReduction { .. } => Icon::new(IconKind::AttackDamage)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .to_rendering_tree(),
        }
    }
}
