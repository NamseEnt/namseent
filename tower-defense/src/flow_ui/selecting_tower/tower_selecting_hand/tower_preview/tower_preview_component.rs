use super::{
    stat::StatPreview,
    tower_skill::{TowerEffectDescription, TowerSkillTemplateIcon},
};
use crate::format_compact_number;
use crate::{
    game_state::{
        self, GameState,
        tower::{TowerSkillTemplate, TowerTemplate},
        upgrade::{TowerUpgradeState, TowerUpgradeTarget, Upgrade, UpgradeBehavior},
    },
    icon::{Icon, IconKind, IconSize},
    theme::typography::{FontSize, memoized_text},
};
use namui::*;
use namui_prebuilt::table;

const PADDING: Px = px(4.0);
const HEADLINE_FONT_SIZE_SMALL: Px = px(16.0);
const PARAGRAPH_FONT_SIZE_LARGE: Px = px(16.0);
const PREVIEW_ICON_SIZE: Px = px(40.0);

#[derive(State)]
struct MouseHoveringSkill {
    skill: TowerSkillTemplate,
    offset: Xy<Px>,
}

pub struct TowerPreviewContent<'a> {
    pub wh: Wh<Px>,
    pub tower_template: &'a TowerTemplate,
}

impl Component for TowerPreviewContent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;

        let (mouse_hovering_effect, set_mouse_hovering_effect) =
            ctx.state::<Option<MouseHoveringSkill>>(|| None);
        let game_state = game_state::use_game_state(ctx);
        let tracked_upgrade_revision = ctx.track_eq(&game_state.upgrade_state.revision);
        let tracked_tower_template = ctx.track_eq(&(
            tower_template.kind,
            tower_template.suit,
            tower_template.rank,
            tower_template.rerolled_count,
        ));
        let upgrade_state_and_texts = ctx.memo(|| {
            tracked_upgrade_revision.record_as_used();
            tracked_tower_template.record_as_used();
            calculate_upgrade_state_and_texts(game_state.as_ref(), tower_template)
        });
        let (upgrade_state, texts) = upgrade_state_and_texts.as_ref();

        let on_mouse_move_in_effect_icon = |effect: &TowerSkillTemplate, offset| {
            set_mouse_hovering_effect.set(Some(MouseHoveringSkill {
                skill: *effect,
                offset,
            }));
        };
        let on_mouse_move_out_effect_icon = |effect: &TowerSkillTemplate| {
            let Some(mouse_hovering_effect) = mouse_hovering_effect.as_ref() else {
                return;
            };
            if &mouse_hovering_effect.skill != effect {
                return;
            }
            set_mouse_hovering_effect.set(None);
        };

        ctx.compose(|ctx| {
            let Some(MouseHoveringSkill {
                skill: effect,
                offset,
            }) = mouse_hovering_effect.as_ref()
            else {
                return;
            };

            ctx.absolute(*offset)
                .add(TowerEffectDescription { skill: effect });
        });

        ctx.compose(|ctx| {
            table::padding_no_clip(PADDING, |wh, ctx| {
                table::vertical([
                    table::fixed_no_clip(HEADLINE_FONT_SIZE_SMALL, |wh, ctx| {
                        let damage = tower_template.default_damage;
                        let damage_plus = *game_state
                            .config
                            .towers
                            .rank_bonus_damage
                            .get(&tower_template.rank)
                            .unwrap_or(&tower_template.rank.bonus_damage())
                            as f32;
                        let damage_multiplier = upgrade_state.damage_multiplier;
                        let attack_power = (damage + damage_plus) * damage_multiplier;
                        let attack_power_text = format_compact_number(attack_power);
                        ctx.compose(|ctx| {
                            let _badge_width = crate::render_attack_power_badge(
                                &ctx,
                                &attack_power_text,
                                wh.width,
                                wh.height,
                            );
                            let _ = _badge_width;
                        });

                        ctx.add(memoized_text(
                            (&wh, &tower_template.kind, &tower_template.suit),
                            |mut builder| {
                                let rank_text = tower_template.rank.to_string();
                                let mut builder = builder.size(FontSize::Small).max_width(wh.width);

                                if !matches!(
                                    tower_template.kind,
                                    crate::game_state::tower::TowerKind::Barricade
                                ) {
                                    builder = builder
                                        .icon(IconKind::Suit {
                                            suit: tower_template.suit,
                                        })
                                        .text(&rank_text)
                                        .space();
                                }

                                builder
                                    .text(game_state.text().tower(tower_template.kind.to_text()))
                                    .render_left_center(wh.height)
                            },
                        ));
                    }),
                    table::fixed_no_clip(PARAGRAPH_FONT_SIZE_LARGE, |wh, ctx| {
                        let rank_bonus = *game_state
                            .config
                            .towers
                            .rank_bonus_damage
                            .get(&tower_template.rank)
                            .unwrap_or(&tower_template.rank.bonus_damage());
                        let rating = tower_template
                            .calculate_rating(upgrade_state.damage_multiplier, rank_bonus);

                        // Follow StatPreview layout: left icon, right-aligned value
                        ctx.add(
                            Icon::new(IconKind::Rating)
                                .size(IconSize::Small)
                                .wh(Wh::new(16.px(), wh.height)),
                        );
                        ctx.add(memoized_text((&rating, &wh.width), |mut builder| {
                            builder
                                .paragraph()
                                .size(FontSize::Medium)
                                .text(format_compact_number(rating))
                                .render_right_top(wh.width)
                        }));
                    }),
                    table::fixed_no_clip(PARAGRAPH_FONT_SIZE_LARGE, |wh, ctx| {
                        let damage = tower_template.default_damage;
                        let damage_plus = *game_state
                            .config
                            .towers
                            .rank_bonus_damage
                            .get(&tower_template.rank)
                            .unwrap_or(&tower_template.rank.bonus_damage())
                            as f32;
                        let damage_multiplier = upgrade_state.damage_multiplier;

                        ctx.add(StatPreview {
                            stat_icon_kind: IconKind::Damage,
                            default_stat: damage,
                            plus_stat: damage_plus,
                            multiplier: damage_multiplier,
                            wh,
                            upgrade_texts: texts.damage.as_slice(),
                        });
                    }),
                    table::fixed_no_clip(
                        PREVIEW_ICON_SIZE,
                        table::horizontal(tower_template.skill_templates.iter().map(|effect| {
                            table::fixed_no_clip(
                                PREVIEW_ICON_SIZE,
                                table::padding_no_clip(PADDING, |wh, ctx| {
                                    ctx.add(TowerSkillTemplateIcon {
                                        skill: effect,
                                        wh,
                                        on_mouse_move_in_effect_icon: &on_mouse_move_in_effect_icon,
                                        on_mouse_move_out_effect_icon:
                                            &on_mouse_move_out_effect_icon,
                                    });
                                }),
                            )
                        })),
                    ),
                ])(wh, ctx);
            })(wh, ctx);
        });
    }
}

#[derive(State)]
struct UpgradeTexts {
    damage: Vec<Upgrade>,
}

fn calculate_upgrade_state_and_texts(
    game_state: &GameState,
    tower_template: &TowerTemplate,
) -> (TowerUpgradeState, UpgradeTexts) {
    let mut state = TowerUpgradeState::default();
    let mut combined_damage_multiplier = 1.0;
    let mut texts = UpgradeTexts { damage: vec![] };

    for upgrade in &game_state.upgrade_state.upgrades {
        let Some((target, bonus_pct)) = upgrade.tower_upgrade_damage_bonus(game_state) else {
            continue;
        };

        if !target_applies_to_tower_template(&target, tower_template) {
            continue;
        }

        let damage_multiplier = if target == TowerUpgradeTarget::RerolledTower {
            (1.0 + bonus_pct).powi(tower_template.rerolled_count as i32)
        } else {
            1.0 + bonus_pct
        };

        combined_damage_multiplier *= damage_multiplier;

        if damage_multiplier > 1.0 {
            texts.damage.push(*upgrade);
        }
    }

    state.damage_multiplier = combined_damage_multiplier;
    (state, texts)
}

fn target_applies_to_tower_template(
    target: &TowerUpgradeTarget,
    tower_template: &TowerTemplate,
) -> bool {
    match target {
        TowerUpgradeTarget::Global => true,
        TowerUpgradeTarget::Suit { suit } => *suit == tower_template.suit,
        TowerUpgradeTarget::EvenOdd { even } => *even == tower_template.rank.is_even(),
        TowerUpgradeTarget::FaceNumber { face } => *face == tower_template.rank.is_face(),
        TowerUpgradeTarget::LowCardTower => tower_template.kind.is_low_card_tower(),
        TowerUpgradeTarget::NoRerollTower => tower_template.rerolled_count == 0,
        TowerUpgradeTarget::RerolledTower => tower_template.rerolled_count > 0,
        TowerUpgradeTarget::TowerId { .. } => false,
    }
}
