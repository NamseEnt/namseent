use super::PADDING;
use crate::{
    card::Rank,
    game_state::tower::{TowerKind, TowerSkillKind, TowerSkillTemplate, TowerTemplate},
    palette,
    theme::typography::{FontSize, Headline, Paragraph, TextAlign},
};
use namui::*;
use namui_prebuilt::{simple_rect, table, typography};

const PREVIEW_ICON_SIZE: Px = px(24.);
const TOWER_EFFECT_DESCRIPTION_MAXWIDTH: Px = px(192.);

pub(super) struct TowerPreview<'a> {
    pub(super) wh: Wh<Px>,
    pub(super) tower_template: &'a TowerTemplate,
}
impl Component for TowerPreview<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;

        let (mouse_hovering_effect, set_mouse_hovering_effect) =
            ctx.state::<Option<MouseHoveringSkill>>(|| None);

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
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        let damage = tower_template.kind.default_damage()
                            + tower_template.rank.bonus_damage();

                        ctx.add(Paragraph {
                            text: "Damage: ".to_string(),
                            font_size: FontSize::Medium,
                            text_align: TextAlign::LeftTop,
                            max_width: None,
                        });
                        ctx.add(Paragraph {
                            text: format!("{damage}"),
                            font_size: FontSize::Medium,
                            text_align: TextAlign::RightTop { width: wh.width },
                            max_width: None,
                        });
                    }),
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        let range = match tower_template.kind {
                            TowerKind::Barricade => "none",
                            TowerKind::High => "normal",
                            TowerKind::OnePair => "normal",
                            TowerKind::TwoPair => "normal",
                            TowerKind::ThreeOfAKind => "normal",
                            TowerKind::Straight => "long",
                            TowerKind::Flush => "normal",
                            TowerKind::FullHouse => "normal",
                            TowerKind::FourOfAKind => "normal",
                            TowerKind::StraightFlush => "long",
                            TowerKind::RoyalFlush => "very long",
                        };

                        ctx.add(Paragraph {
                            text: "Range: ".to_string(),
                            font_size: FontSize::Medium,
                            text_align: TextAlign::LeftTop,
                            max_width: None,
                        });
                        ctx.add(Paragraph {
                            text: range.to_string(),
                            font_size: FontSize::Medium,
                            text_align: TextAlign::RightTop { width: wh.width },
                            max_width: None,
                        });
                    }),
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        let speed = match tower_template.kind {
                            TowerKind::Barricade => "none",
                            TowerKind::High => "normal",
                            TowerKind::OnePair => "normal",
                            TowerKind::TwoPair => "normal",
                            TowerKind::ThreeOfAKind => "normal",
                            TowerKind::Straight => "normal",
                            TowerKind::Flush => "fast",
                            TowerKind::FullHouse => "normal",
                            TowerKind::FourOfAKind => "normal",
                            TowerKind::StraightFlush => "fast",
                            TowerKind::RoyalFlush => "very fast",
                        };

                        ctx.add(Paragraph {
                            text: "Speed: ".to_string(),
                            font_size: FontSize::Medium,
                            text_align: TextAlign::LeftTop,
                            max_width: None,
                        });
                        ctx.add(Paragraph {
                            text: speed.to_string(),
                            font_size: FontSize::Medium,
                            text_align: TextAlign::RightTop { width: wh.width },
                            max_width: None,
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

pub struct TowerSkillTemplateIcon<'a> {
    wh: Wh<Px>,
    skill: &'a TowerSkillTemplate,
    on_mouse_move_in_effect_icon: &'a dyn Fn(&TowerSkillTemplate, Xy<Px>),
    on_mouse_move_out_effect_icon: &'a dyn Fn(&TowerSkillTemplate),
}
// TODO: Use image instead of text
impl Component for TowerSkillTemplateIcon<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            skill,
            on_mouse_move_in_effect_icon,
            on_mouse_move_out_effect_icon,
        } = self;
        let symbol = match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul { .. } => "E",
            TowerSkillKind::NearbyTowerDamageAdd { .. } => "E",
            TowerSkillKind::NearbyTowerAttackSpeedAdd { .. } => "H",
            TowerSkillKind::NearbyTowerAttackSpeedMul { .. } => "H",
            TowerSkillKind::NearbyTowerAttackRangeAdd { .. } => "R",
            TowerSkillKind::NearbyMonsterSpeedMul { .. } => "D",
            TowerSkillKind::MoneyIncomeAdd { .. } => "B",
            TowerSkillKind::TopCardBonus { rank, .. } => match rank {
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
            },
        };
        ctx.add(typography::body::center(wh, symbol, palette::ON_SURFACE));
        ctx.add(simple_rect(
            wh,
            palette::OUTLINE,
            1.px(),
            palette::SURFACE_CONTAINER_HIGH,
        ))
        .attach_event(|event| {
            match event {
                Event::MouseMove { event } => {
                    match event.is_local_xy_in() {
                        true => on_mouse_move_in_effect_icon(skill, event.global_xy),
                        false => on_mouse_move_out_effect_icon(skill),
                    };
                }
                Event::VisibilityChange => {
                    on_mouse_move_out_effect_icon(skill);
                }
                _ => {}
            };
        });
    }
}

pub struct TowerEffectDescription<'a> {
    skill: &'a TowerSkillTemplate,
}
impl Component for TowerEffectDescription<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { skill } = self;

        let title = match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul { .. } => "주변 타워 공격력 증가".to_string(),
            TowerSkillKind::NearbyTowerDamageAdd { .. } => "주변 타워 공격력 추가".to_string(),
            TowerSkillKind::NearbyTowerAttackSpeedAdd { .. } => {
                "주변 타워 공격 속도 추가".to_string()
            }
            TowerSkillKind::NearbyTowerAttackSpeedMul { .. } => {
                "주변 타워 공격 속도 증가".to_string()
            }
            TowerSkillKind::NearbyTowerAttackRangeAdd { .. } => {
                "주변 타워 공격 범위 추가".to_string()
            }
            TowerSkillKind::NearbyMonsterSpeedMul { .. } => "주변 몬스터 속도 감소".to_string(),
            TowerSkillKind::MoneyIncomeAdd { .. } => "돈 수입 증가".to_string(),
            TowerSkillKind::TopCardBonus { .. } => "탑 카드 보너스".to_string(),
        };
        let description = match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul { mul, range_radius } => {
                format!(
                    "주변 타워의 공격력을 {}% 증가시킵니다 (반경 {} 타일)",
                    mul * 100.0,
                    range_radius
                )
            }
            TowerSkillKind::NearbyTowerDamageAdd { add, range_radius } => {
                format!(
                    "주변 타워의 공격력을 {}만큼 증가시킵니다 (반경 {} 타일)",
                    add, range_radius
                )
            }
            TowerSkillKind::NearbyTowerAttackSpeedAdd { add, range_radius } => {
                format!(
                    "주변 타워의 공격 속도를 {}% 증가시킵니다 (반경 {} 타일)",
                    add * 100.0,
                    range_radius
                )
            }
            TowerSkillKind::NearbyTowerAttackSpeedMul { mul, range_radius } => {
                format!(
                    "주변 타워의 공격 속도를 {}배 증가시킵니다 (반경 {} 타일)",
                    mul, range_radius
                )
            }
            TowerSkillKind::NearbyTowerAttackRangeAdd { add, range_radius } => {
                format!(
                    "주변 타워의 공격 범위를 {} 타일 증가시킵니다 (반경 {} 타일)",
                    add, range_radius
                )
            }
            TowerSkillKind::NearbyMonsterSpeedMul { mul, range_radius } => {
                format!(
                    "주변 몬스터의 속도를 {}% 감소시킵니다 (반경 {} 타일)",
                    mul * 100.0,
                    range_radius
                )
            }
            TowerSkillKind::MoneyIncomeAdd { add } => {
                format!("적 처치시 {} 골드를 추가로 획득합니다", add)
            }
            TowerSkillKind::TopCardBonus { rank, bonus_damage } => {
                format!("탑 카드 보너스: {} (공격력 +{})", rank, bonus_damage)
            }
        };

        ctx.compose(|ctx| {
            let text_content = ctx.ghost_compose("TowerEffect description tooltip", |ctx| {
                table::vertical([
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        ctx.add(Headline {
                            text: title,
                            font_size: FontSize::Small,
                            text_align: TextAlign::LeftTop,
                            max_width: Some(TOWER_EFFECT_DESCRIPTION_MAXWIDTH),
                        });
                    }),
                    table::fixed(PADDING, |_, _| {}),
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        ctx.add(Paragraph {
                            text: description,
                            font_size: FontSize::Medium,
                            text_align: TextAlign::LeftTop,
                            max_width: Some(TOWER_EFFECT_DESCRIPTION_MAXWIDTH),
                        });
                    }),
                ])(
                    Wh {
                        width: TOWER_EFFECT_DESCRIPTION_MAXWIDTH,
                        height: f32::MAX.px(),
                    },
                    ctx,
                );
            });

            let Some(text_content_wh) = bounding_box(&text_content).map(|rect| rect.wh()) else {
                return;
            };

            let ctx = ctx.translate((0.px(), -text_content_wh.height - PADDING * 2.0));

            ctx.translate(Xy::single(PADDING)).add(text_content);

            ctx.add(simple_rect(
                text_content_wh + Wh::single(PADDING * 2.0),
                palette::OUTLINE,
                1.px(),
                palette::SURFACE_CONTAINER_HIGH,
            ));
        });
    }
}

#[derive(Clone, Copy, PartialEq)]
struct MouseHoveringSkill {
    skill: TowerSkillTemplate,
    offset: Xy<Px>,
}
