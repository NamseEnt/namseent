use super::PADDING;
use crate::{
    card::Rank,
    palette,
    tower::{TowerBlueprint, TowerEffectBlueprint},
};
use namui::*;
use namui_prebuilt::{simple_rect, table, typography};

const PREVIEW_ICON_SIZE: Px = px(24.);
const TOWER_EFFECT_DESCRIPTION_WIDTH: Px = px(256.);

pub(super) struct TowerPreview<'a> {
    pub(super) wh: Wh<Px>,
    pub(super) tower_blueprint: &'a TowerBlueprint,
}
impl Component for TowerPreview<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            tower_blueprint,
        } = self;

        let (mouse_hovering_effect, set_mouse_hovering_effect) =
            ctx.state::<Option<MouseHoveringEffect>>(|| None);

        let on_mouse_move_in_effect_icon = |effect: &TowerEffectBlueprint, offset| {
            set_mouse_hovering_effect.set(Some(MouseHoveringEffect {
                effect: *effect,
                offset,
            }));
        };
        let on_mouse_move_out_effect_icon = |effect: &TowerEffectBlueprint| {
            let Some(mouse_hovering_effect) = mouse_hovering_effect.as_ref() else {
                return;
            };
            if &mouse_hovering_effect.effect != effect {
                return;
            }
            set_mouse_hovering_effect.set(None);
        };

        ctx.compose(|ctx| {
            let Some(MouseHoveringEffect { effect, offset }) = mouse_hovering_effect.as_ref()
            else {
                return;
            };

            ctx.absolute(*offset)
                .add(TowerEffectDescription { effect: &effect });
        });

        ctx.compose(|ctx| {
            table::padding_no_clip(
                PADDING,
                table::vertical([
                    table::fixed_no_clip(typography::body::FONT_SIZE.into_px(), |wh, ctx| {
                        let mut tower_name = String::new();
                        if let Some(suit) = tower_blueprint.suit {
                            tower_name.push_str(&format!("{}", suit));
                        }
                        if let Some(rank) = tower_blueprint.rank {
                            tower_name.push_str(&format!("{}", rank));
                        }
                        tower_name.push_str(&format!(" {:?}", tower_blueprint.kind));

                        ctx.add(typography::body::left(
                            wh.height,
                            tower_name,
                            palette::ON_SURFACE,
                        ));
                    }),
                    table::fixed_no_clip(typography::body::FONT_SIZE.into_px(), |wh, ctx| {
                        let damage = tower_blueprint.calculate_damage();

                        ctx.add(typography::body::left(
                            wh.height,
                            "Damage: ",
                            palette::ON_SURFACE_VARIANT,
                        ));
                        ctx.add(typography::body::right(
                            wh,
                            format!("{damage}"),
                            palette::ON_SURFACE,
                        ));
                    }),
                    table::fixed_no_clip(typography::body::FONT_SIZE.into_px(), |wh, ctx| {
                        let range = match tower_blueprint.kind {
                            crate::tower::TowerKind::High => "normal",
                            crate::tower::TowerKind::OnePair => "normal",
                            crate::tower::TowerKind::TwoPair => "normal",
                            crate::tower::TowerKind::ThreeOfAKind => "normal",
                            crate::tower::TowerKind::Straight => "long",
                            crate::tower::TowerKind::Flush => "normal",
                            crate::tower::TowerKind::FullHouse => "normal",
                            crate::tower::TowerKind::FourOfAKind => "normal",
                            crate::tower::TowerKind::StraightFlush => "long",
                            crate::tower::TowerKind::RoyalFlush => "very long",
                        };

                        ctx.add(typography::body::left(
                            wh.height,
                            "Range: ",
                            palette::ON_SURFACE_VARIANT,
                        ));
                        ctx.add(typography::body::right(wh, range, palette::ON_SURFACE));
                    }),
                    table::fixed_no_clip(typography::body::FONT_SIZE.into_px(), |wh, ctx| {
                        let speed = match tower_blueprint.kind {
                            crate::tower::TowerKind::High => "normal",
                            crate::tower::TowerKind::OnePair => "normal",
                            crate::tower::TowerKind::TwoPair => "normal",
                            crate::tower::TowerKind::ThreeOfAKind => "normal",
                            crate::tower::TowerKind::Straight => "normal",
                            crate::tower::TowerKind::Flush => "fast",
                            crate::tower::TowerKind::FullHouse => "normal",
                            crate::tower::TowerKind::FourOfAKind => "normal",
                            crate::tower::TowerKind::StraightFlush => "fast",
                            crate::tower::TowerKind::RoyalFlush => "very fast",
                        };

                        ctx.add(typography::body::left(
                            wh.height,
                            "Speed: ",
                            palette::ON_SURFACE_VARIANT,
                        ));
                        ctx.add(typography::body::right(wh, speed, palette::ON_SURFACE));
                    }),
                    table::fixed_no_clip(
                        PREVIEW_ICON_SIZE,
                        table::horizontal(tower_blueprint.effects.iter().map(|effect| {
                            table::fixed_no_clip(
                                PREVIEW_ICON_SIZE,
                                table::padding_no_clip(PADDING, |wh, ctx| {
                                    ctx.add(TowerEffectBlueprintIcon {
                                        effect,
                                        wh,
                                        on_mouse_move_in_effect_icon: &on_mouse_move_in_effect_icon,
                                        on_mouse_move_out_effect_icon:
                                            &on_mouse_move_out_effect_icon,
                                    });
                                }),
                            )
                        })),
                    ),
                ]),
            )(wh, ctx);
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

pub struct TowerEffectBlueprintIcon<'a> {
    wh: Wh<Px>,
    effect: &'a TowerEffectBlueprint,
    on_mouse_move_in_effect_icon: &'a dyn Fn(&TowerEffectBlueprint, Xy<Px>),
    on_mouse_move_out_effect_icon: &'a dyn Fn(&TowerEffectBlueprint),
}
impl Component for TowerEffectBlueprintIcon<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            effect,
            on_mouse_move_in_effect_icon,
            on_mouse_move_out_effect_icon,
        } = self;
        let symbol = match effect {
            TowerEffectBlueprint::TopCardBonus { rank, .. } => match rank {
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
            },
            TowerEffectBlueprint::Bounty { .. } => "B",
            TowerEffectBlueprint::Drag { .. } => "D",
            TowerEffectBlueprint::Haste { .. } => "H",
            TowerEffectBlueprint::Empower { .. } => "E",
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
                        true => on_mouse_move_in_effect_icon(effect, event.global_xy),
                        false => on_mouse_move_out_effect_icon(effect),
                    };
                }
                Event::VisibilityChange => {
                    on_mouse_move_out_effect_icon(effect);
                }
                _ => {}
            };
        });
    }
}

pub struct TowerEffectDescription<'a> {
    effect: &'a TowerEffectBlueprint,
}
impl Component for TowerEffectDescription<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { effect } = self;

        let title = match effect {
            TowerEffectBlueprint::TopCardBonus { rank, .. } => format!("Top Card Bonus {rank}"),
            TowerEffectBlueprint::Bounty { .. } => "Bounty".to_string(),
            TowerEffectBlueprint::Drag { .. } => "Drag".to_string(),
            TowerEffectBlueprint::Haste { .. } => "Haste".to_string(),
            TowerEffectBlueprint::Empower { .. } => "Empower".to_string(),
        };
        let description = match effect {
            TowerEffectBlueprint::TopCardBonus { bonus_damage, .. } => {
                format!("공격력 +{bonus_damage}")
            }
            TowerEffectBlueprint::Bounty { bonus_gold } => {
                format!("적을 처치할 때 보너스 골드 {bonus_gold}를 획득합니다")
            }
            TowerEffectBlueprint::Drag { range, drag } => {
                format!("주변 {range} 타일 내 적의 이동속도를 {drag}배 둔화시킵니다")
            }
            TowerEffectBlueprint::Haste { range, haste } => {
                format!("주변 {range} 타일 내 타워의 공격속도를 {haste}배 증가시킵니다")
            }
            TowerEffectBlueprint::Empower { range, empower } => {
                format!("주변 {range} 타일 내 타워의 공격력을 {empower}배 증가시킵니다")
            }
        };

        ctx.compose(|ctx| {
            let height = PADDING * 3. + typography::body::FONT_SIZE.into_px() * 2.;
            let ctx = ctx.translate((0.px(), -height));
            ctx.compose(|ctx| {
                table::padding(
                    PADDING,
                    table::vertical([
                        table::fixed_no_clip(
                            typography::body::FONT_SIZE.into_px(),
                            |_wh: Wh<Px>, ctx| {
                                ctx.add(typography::body::left_top(title, palette::ON_SURFACE));
                            },
                        ),
                        table::ratio(1, |_, _| {}),
                        table::fixed_no_clip(typography::body::FONT_SIZE.into_px(), |_wh, ctx| {
                            ctx.add(typography::body::left_top(
                                description,
                                palette::ON_SURFACE_VARIANT,
                            ));
                        }),
                    ]),
                )(
                    Wh {
                        width: TOWER_EFFECT_DESCRIPTION_WIDTH,
                        height,
                    },
                    ctx,
                );
            });

            ctx.add(simple_rect(
                Wh {
                    width: TOWER_EFFECT_DESCRIPTION_WIDTH,
                    height,
                },
                palette::OUTLINE,
                1.px(),
                palette::SURFACE_CONTAINER_HIGH,
            ));
        });
    }
}

#[derive(Clone, Copy, PartialEq)]
struct MouseHoveringEffect {
    effect: TowerEffectBlueprint,
    offset: Xy<Px>,
}
