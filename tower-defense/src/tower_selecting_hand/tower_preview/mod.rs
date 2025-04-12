mod stat;
mod tower_skill;

use crate::{
    game_state::{
        self, GameState,
        tower::{TowerSkillTemplate, TowerTemplate},
        upgrade::{TowerSelectUpgradeTarget, TowerUpgradeState, TowerUpgradeTarget},
    },
    palette,
    theme::typography::{FontSize, Headline, PARAGRAPH_FONT_SIZE_MEDIUM, TextAlign},
};
use namui::*;
use namui_prebuilt::table;
use stat::StatPreview;
use tower_skill::{MouseHoveringSkill, TowerEffectDescription, TowerSkillTemplateIcon};

use super::PADDING;

const PREVIEW_ICON_SIZE: Px = px(24.);

pub(super) struct TowerPreview<'a> {
    pub(super) wh: Wh<Px>,
    pub(super) tower_template: &'a TowerTemplate,
}
impl Component for TowerPreview<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;

        let (mouse_hovering_effect, set_mouse_hovering_effect) =
            ctx.state::<Option<MouseHoveringSkill>>(|| None);
        let game_state = game_state::use_game_state(ctx);
        let upgrade_state =
            ctx.memo(|| calculate_upgrade_state(game_state.as_ref(), tower_template));

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
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        let mut tower_name = String::new();
                        tower_name.push_str(&format!("{}", tower_template.suit));
                        tower_name.push_str(&format!("{}", tower_template.rank));
                        tower_name.push_str(&format!(" {:?}", tower_template.kind));

                        ctx.add(Headline {
                            text: tower_name,
                            font_size: FontSize::Small,
                            text_align: TextAlign::LeftTop,
                            max_width: Some(wh.width),
                        });
                    }),
                    table::fixed_no_clip(PARAGRAPH_FONT_SIZE_MEDIUM.into_px(), |wh, ctx| {
                        let damage = tower_template.kind.default_damage();
                        let damage_plus =
                            upgrade_state.damage_plus + tower_template.rank.bonus_damage() as f32;
                        let damage_multiplier = upgrade_state.damage_multiplier;

                        ctx.add(StatPreview {
                            stat_name: "Damage",
                            default_stat: damage as f32,
                            plus_stat: damage_plus,
                            multiplier: damage_multiplier,
                            wh,
                            // UpgradeKind::description(), upgrade_board::get_upgrade_description_texts()
                            upgrade_texts: vec![
                                "랭크가 10인 타워의 공격력이 2.3배 증가합니다".to_string(),
                                "짝수 타워의 공격력이 2.3배 증가합니다".to_string(),
                                "숫자 타워의 공격력이 2.3배 증가합니다".to_string(),
                                "리롤하지 않고 타워를 만들면 타워의 공격력이 2.3배 증가합니다."
                                    .to_string(),
                            ],
                        });
                    }),
                    table::fixed_no_clip(PARAGRAPH_FONT_SIZE_MEDIUM.into_px(), |wh, ctx| {
                        let range = tower_template.default_attack_range_radius;
                        let range_plus = upgrade_state.range_plus;

                        ctx.add(StatPreview {
                            stat_name: "Range",
                            default_stat: range,
                            plus_stat: range_plus,
                            multiplier: 1.0, // No multiplier for range
                            wh,
                            upgrade_texts: vec![],
                        });
                    }),
                    table::fixed_no_clip(PARAGRAPH_FONT_SIZE_MEDIUM.into_px(), |wh, ctx| {
                        let attack_speed = 1.0 / tower_template.kind.shoot_interval().as_secs_f32();
                        let speed_plus = upgrade_state.speed_plus;
                        let speed_multiplier = upgrade_state.speed_multiplier;

                        ctx.add(StatPreview {
                            stat_name: "Speed",
                            default_stat: attack_speed,
                            plus_stat: speed_plus,
                            multiplier: speed_multiplier,
                            wh,
                            upgrade_texts: vec![
                                "랭크가 10인 타워의 공격속도가가 2.3배 증가합니다".to_string(),
                                "짝수 타워의 공격속도가가 2.3배 증가합니다".to_string(),
                                "숫자 타워의 공격속도가가 2.3배 증가합니다".to_string(),
                            ],
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

fn calculate_upgrade_state(
    game_state: &GameState,
    tower_template: &TowerTemplate,
) -> TowerUpgradeState {
    let mut state = TowerUpgradeState::default();
    let mut apply_upgrade = |upgrade_state: &TowerUpgradeState| {
        state.damage_plus += upgrade_state.damage_plus;
        state.damage_multiplier *= upgrade_state.damage_multiplier;
        state.speed_plus += upgrade_state.speed_plus;
        state.speed_multiplier *= upgrade_state.speed_multiplier;
        state.range_plus += upgrade_state.range_plus;
    };

    let apply_tower_upgrade_target = |target| {
        let Some(upgrade_state) = game_state.upgrade_state.tower_upgrade_states.get(&target) else {
            return;
        };
        apply_upgrade(upgrade_state);
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
        apply_upgrade(upgrade_state);
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
            apply_upgrade(upgrade_state);
        }
    }

    state
}
