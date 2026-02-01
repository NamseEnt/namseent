use super::format_compact_number;
use super::{
    stat::StatPreview,
    tower_skill::{TowerEffectDescription, TowerSkillTemplateIcon},
    upgrade_helpers::*,
};
use crate::{
    game_state::{
        self, GameState,
        tower::{TowerSkillTemplate, TowerTemplate},
        upgrade::{TowerSelectUpgradeTarget, TowerUpgradeState, TowerUpgradeTarget},
    },
    icon::{Icon, IconKind, IconSize},
    palette,
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
        let upgrade_state_and_texts =
            ctx.memo(|| calculate_upgrade_state_and_texts(game_state.as_ref(), tower_template));
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
                                        .icon::<()>(IconKind::Suit {
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
                        let rating =
                            tower_template.calculate_rating(upgrade_state.damage_multiplier);

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
                        let damage = tower_template.kind.default_damage();
                        let damage_plus = tower_template.rank.bonus_damage() as f32;
                        let damage_multiplier = upgrade_state.damage_multiplier;

                        ctx.add(StatPreview {
                            stat_icon_kind: IconKind::AttackDamage,
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

pub struct TowerPreview<'a> {
    pub wh: Wh<Px>,
    pub tower_template: &'a TowerTemplate,
}

impl Component for TowerPreview<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;

        ctx.add(TowerPreviewContent { wh, tower_template });

        ctx.add(rect(RectParam {
            rect: wh.to_rect(),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill {
                    color: palette::SURFACE,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));
    }
}

#[derive(State)]
struct UpgradeTexts {
    damage: Vec<crate::game_state::upgrade::UpgradeKind>,
    speed: Vec<crate::game_state::upgrade::UpgradeKind>,
}

fn calculate_upgrade_state_and_texts(
    game_state: &GameState,
    tower_template: &TowerTemplate,
) -> (TowerUpgradeState, UpgradeTexts) {
    let mut state = TowerUpgradeState::default();
    let mut texts = UpgradeTexts {
        damage: vec![],
        speed: vec![],
    };

    let mut apply_upgrade = |upgrade_state: &TowerUpgradeState, target: &UpgradeTargetType| {
        state.damage_multiplier *= upgrade_state.damage_multiplier;

        if upgrade_state.damage_multiplier > 1.0 {
            let upgrade_kind = match target {
                UpgradeTargetType::Tower(tower_target) => create_upgrade_kind_for_target(
                    tower_target,
                    UpgradeStatType::Damage,
                    false,
                    upgrade_state.damage_multiplier,
                ),
                UpgradeTargetType::TowerSelect(tower_select_target) => {
                    create_tower_select_upgrade_kind(
                        tower_select_target,
                        UpgradeStatType::Damage,
                        false,
                        upgrade_state.damage_multiplier,
                    )
                }
            };
            texts.damage.push(upgrade_kind);
        }
    };

    let apply_tower_upgrade_target = |target| {
        let Some(upgrade_state) = game_state.upgrade_state.tower_upgrade_states.get(&target) else {
            return;
        };
        apply_upgrade(upgrade_state, &UpgradeTargetType::Tower(target));
    };

    let targets = [
        TowerUpgradeTarget::Rank {
            rank: tower_template.rank,
        },
        TowerUpgradeTarget::Suit {
            suit: tower_template.suit,
        },
        TowerUpgradeTarget::TowerKind {
            tower_kind: tower_template.kind,
        },
        TowerUpgradeTarget::EvenOdd {
            even: tower_template.rank.is_even(),
        },
        TowerUpgradeTarget::FaceNumber {
            face: tower_template.rank.is_face(),
        },
    ];
    targets.into_iter().for_each(apply_tower_upgrade_target);

    let mut apply_tower_select_upgrade_target = |target| {
        let Some(upgrade_state) = game_state
            .upgrade_state
            .tower_select_upgrade_states
            .get(&target)
        else {
            return;
        };
        apply_upgrade(upgrade_state, &UpgradeTargetType::TowerSelect(target));
    };

    if tower_template.kind.is_low_card_tower() {
        apply_tower_select_upgrade_target(TowerSelectUpgradeTarget::LowCard);
    }

    let rerolled_count = game_state.rerolled_count;
    if rerolled_count == 0 {
        apply_tower_select_upgrade_target(TowerSelectUpgradeTarget::NoReroll);
    } else if let Some(upgrade_state) = game_state
        .upgrade_state
        .tower_select_upgrade_states
        .get(&TowerSelectUpgradeTarget::Reroll)
    {
        for _ in 0..rerolled_count {
            apply_upgrade(
                upgrade_state,
                &UpgradeTargetType::TowerSelect(TowerSelectUpgradeTarget::Reroll),
            );
        }
    }

    (state, texts)
}
