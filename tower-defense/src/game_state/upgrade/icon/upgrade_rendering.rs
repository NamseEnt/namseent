use crate::{
    game_state::upgrade::UpgradeKind,
    icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize},
};
use namui::*;

use super::{
    constants::{OVERLAY_SIZE_RATIO, RANK_OVERLAY_SIZE_RATIO},
    overlay_functions::{
        render_even_odd_indicator, render_expansion_overlay, render_face_number_indicator,
        render_low_card_indicator, render_no_reroll_indicator, render_plus_overlay,
        render_rank_overlay, render_reroll_indicator, render_same_suits_indicator,
        render_shortcut_indicator, render_skip_indicator, render_suit_overlay,
    },
    render_functions::{render_barricade_tower, render_tower_kind_base},
};

impl UpgradeKind {
    pub fn icon(&self, wh: Wh<Px>) -> RenderingTree {
        match self {
            UpgradeKind::GoldEarnPlus => namui::render([
                render_plus_overlay(wh),
                Icon::new(IconKind::Gold)
                    .wh(wh)
                    .size(IconSize::Custom { size: wh.width })
                    .to_rendering_tree(),
            ]),

            // Rank-based upgrades
            UpgradeKind::RankAttackDamagePlus { rank, .. } => namui::render([
                render_rank_overlay(wh, *rank),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::RankAttackDamageMultiply { rank, .. } => namui::render([
                render_rank_overlay(wh, *rank),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::RankAttackSpeedPlus { rank, .. } => namui::render([
                render_rank_overlay(wh, *rank),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::RankAttackSpeedMultiply { rank, .. } => namui::render([
                render_rank_overlay(wh, *rank),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::RankAttackRangePlus { rank, .. } => namui::render([
                render_rank_overlay(wh, *rank),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackRange)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),

            // Suit-based upgrades
            UpgradeKind::SuitAttackDamagePlus { suit, .. } => namui::render([
                render_suit_overlay(wh, *suit),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::SuitAttackDamageMultiply { suit, .. } => namui::render([
                render_suit_overlay(wh, *suit),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::SuitAttackSpeedPlus { suit, .. } => namui::render([
                render_suit_overlay(wh, *suit),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::SuitAttackSpeedMultiply { suit, .. } => namui::render([
                render_suit_overlay(wh, *suit),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::SuitAttackRangePlus { suit, .. } => namui::render([
                render_suit_overlay(wh, *suit),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackRange)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),

            // Hand/Tower kind upgrades
            UpgradeKind::HandAttackDamagePlus { tower_kind, .. } => namui::render([
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_tower_kind_base(wh, *tower_kind),
            ]),
            UpgradeKind::HandAttackDamageMultiply { tower_kind, .. } => namui::render([
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_tower_kind_base(wh, *tower_kind),
            ]),
            UpgradeKind::HandAttackSpeedPlus { tower_kind, .. } => namui::render([
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_tower_kind_base(wh, *tower_kind),
            ]),
            UpgradeKind::HandAttackSpeedMultiply { tower_kind, .. } => namui::render([
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_tower_kind_base(wh, *tower_kind),
            ]),
            UpgradeKind::HandAttackRangePlus { tower_kind, .. } => namui::render([
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::AttackRange)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_tower_kind_base(wh, *tower_kind),
            ]),

            // Expansion upgrades
            UpgradeKind::ShopSlotExpansion => Icon::new(IconKind::Shop)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            UpgradeKind::QuestSlotExpansion => Icon::new(IconKind::Quest)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            UpgradeKind::QuestBoardExpansion => namui::render([
                render_expansion_overlay(wh, "Board"),
                Icon::new(IconKind::Quest)
                    .wh(wh)
                    .size(IconSize::Custom { size: wh.width })
                    .to_rendering_tree(),
            ]),
            UpgradeKind::RerollCountPlus => namui::render([
                render_plus_overlay(wh),
                Icon::new(IconKind::Refresh)
                    .wh(wh)
                    .size(IconSize::Custom { size: wh.width })
                    .to_rendering_tree(),
            ]),

            // Low card upgrades
            UpgradeKind::LowCardTowerDamagePlus { .. } => namui::render([
                render_low_card_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::LowCardTowerDamageMultiply { .. } => namui::render([
                render_low_card_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::LowCardTowerAttackSpeedPlus { .. } => namui::render([
                render_low_card_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::LowCardTowerAttackSpeedMultiply { .. } => namui::render([
                render_low_card_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::LowCardTowerAttackRangePlus { .. } => namui::render([
                render_low_card_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackRange)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),

            // Shop upgrades
            UpgradeKind::ShopItemPriceMinus => Icon::new(IconKind::Shop)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Down).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            UpgradeKind::ShopRefreshPlus => namui::render([
                render_plus_overlay(wh),
                Icon::new(IconKind::Shop)
                    .wh(wh)
                    .size(IconSize::Custom { size: wh.width })
                    .attributes(vec![
                        IconAttribute::new(IconKind::Refresh)
                            .position(IconAttributePosition::BottomRight),
                    ])
                    .to_rendering_tree(),
            ]),
            UpgradeKind::QuestBoardRefreshPlus => namui::render([
                render_plus_overlay(wh),
                Icon::new(IconKind::Quest)
                    .wh(wh)
                    .size(IconSize::Custom { size: wh.width })
                    .attributes(vec![
                        IconAttribute::new(IconKind::Refresh)
                            .position(IconAttributePosition::BottomRight),
                    ])
                    .to_rendering_tree(),
            ]),

            // No-reroll upgrades
            UpgradeKind::NoRerollTowerAttackDamagePlus { .. } => namui::render([
                render_no_reroll_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => namui::render([
                render_no_reroll_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::NoRerollTowerAttackSpeedPlus { .. } => namui::render([
                render_no_reroll_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::NoRerollTowerAttackSpeedMultiply { .. } => namui::render([
                render_no_reroll_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::NoRerollTowerAttackRangePlus { .. } => namui::render([
                render_no_reroll_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackRange)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),

            // Even/Odd upgrades
            UpgradeKind::EvenOddTowerAttackDamagePlus { even, .. } => namui::render([
                render_even_odd_indicator(wh, *even),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::EvenOddTowerAttackDamageMultiply { even, .. } => namui::render([
                render_even_odd_indicator(wh, *even),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::EvenOddTowerAttackSpeedPlus { even, .. } => namui::render([
                render_even_odd_indicator(wh, *even),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::EvenOddTowerAttackSpeedMultiply { even, .. } => namui::render([
                render_even_odd_indicator(wh, *even),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::EvenOddTowerAttackRangePlus { even, .. } => namui::render([
                render_even_odd_indicator(wh, *even),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackRange)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),

            // Face/Number card upgrades
            UpgradeKind::FaceNumberCardTowerAttackDamagePlus { face, .. } => namui::render([
                render_face_number_indicator(wh, *face),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, .. } => namui::render([
                render_face_number_indicator(wh, *face),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { face, .. } => namui::render([
                render_face_number_indicator(wh, *face),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply { face, .. } => namui::render([
                render_face_number_indicator(wh, *face),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::FaceNumberCardTowerAttackRangePlus { face, .. } => namui::render([
                render_face_number_indicator(wh, *face),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackRange)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),

            // Special card rule upgrades
            UpgradeKind::ShortenStraightFlushTo4Cards => namui::render([
                render_shortcut_indicator(wh, "4"),
                Icon::new(IconKind::Card)
                    .wh(wh)
                    .size(IconSize::Custom { size: wh.width })
                    .to_rendering_tree(),
            ]),
            UpgradeKind::SkipRankForStraight => namui::render([
                render_skip_indicator(wh),
                Icon::new(IconKind::Card)
                    .wh(wh)
                    .size(IconSize::Custom { size: wh.width })
                    .to_rendering_tree(),
            ]),
            UpgradeKind::TreatSuitsAsSame => namui::render([
                render_same_suits_indicator(wh),
                Icon::new(IconKind::Card)
                    .wh(wh)
                    .size(IconSize::Custom { size: wh.width })
                    .to_rendering_tree(),
            ]),

            // Reroll upgrades
            UpgradeKind::RerollTowerAttackDamagePlus { .. } => namui::render([
                render_reroll_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::RerollTowerAttackDamageMultiply { .. } => namui::render([
                render_reroll_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackDamage)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::RerollTowerAttackSpeedPlus { .. } => namui::render([
                render_reroll_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::RerollTowerAttackSpeedMultiply { .. } => namui::render([
                render_reroll_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackSpeed)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Multiply)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
            UpgradeKind::RerollTowerAttackRangePlus { .. } => namui::render([
                render_reroll_indicator(wh),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    wh.height * RANK_OVERLAY_SIZE_RATIO,
                    Icon::new(IconKind::AttackRange)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                namui::translate(
                    wh.width * RANK_OVERLAY_SIZE_RATIO,
                    0.px(),
                    Icon::new(IconKind::Add)
                        .wh(wh * OVERLAY_SIZE_RATIO)
                        .size(IconSize::Custom {
                            size: wh.width * OVERLAY_SIZE_RATIO,
                        })
                        .to_rendering_tree(),
                ),
                render_barricade_tower(wh),
            ]),
        }
    }
}
