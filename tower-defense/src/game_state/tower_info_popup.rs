use super::{Tower, mutate_game_state, upgrade::TowerUpgradeState};
use crate::theme::{
    button::{Button, ButtonColor, ButtonVariant},
    palette,
    typography::{FontSize, PARAGRAPH_FONT_SIZE_MEDIUM, TextAlign, paragraph},
};
use namui::*;
use namui_prebuilt::table;

const BUBBLE_PADDING: Px = px(12.);
const BUBBLE_WIDTH: Px = px(280.);
const BUBBLE_HEIGHT: Px = px(200.);

pub struct TowerInfoPopup<'a> {
    pub tower: &'a Tower,
    pub tower_upgrades: &'a [TowerUpgradeState],
    pub game_state: &'a super::GameState,
}

impl Component for TowerInfoPopup<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            tower,
            tower_upgrades,
            game_state,
        } = self;

        ctx.translate((-BUBBLE_WIDTH * 0.5, -BUBBLE_HEIGHT))
            .compose(|ctx| {
                ctx.compose(|ctx| {
                    table::padding(BUBBLE_PADDING, |wh, ctx| {
                        table::vertical([
                            table::fixed(PARAGRAPH_FONT_SIZE_MEDIUM.into_px() * 2.0, |wh, ctx| {
                                ctx.add(
                                    paragraph(format!("{} {}", tower.suit, tower.rank))
                                        .size(FontSize::Medium)
                                        .align(TextAlign::LeftTop)
                                        .max_width(wh.width)
                                        .build(),
                                );
                            }),
                            table::fixed(PARAGRAPH_FONT_SIZE_MEDIUM.into_px(), |wh, ctx| {
                                let damage = tower.calculate_projectile_damage(tower_upgrades, 1.0);
                                ctx.add(
                                    paragraph(format!("데미지: {damage:.1}"))
                                        .size(FontSize::Medium)
                                        .align(TextAlign::LeftTop)
                                        .max_width(wh.width)
                                        .build(),
                                );
                            }),
                            table::fixed(PARAGRAPH_FONT_SIZE_MEDIUM.into_px(), |wh, ctx| {
                                ctx.add(
                                    paragraph(format!(
                                        "속도: {:.2}s",
                                        tower.shoot_interval.as_secs_f32()
                                    ))
                                    .size(FontSize::Medium)
                                    .align(TextAlign::LeftTop)
                                    .max_width(wh.width)
                                    .build(),
                                );
                            }),
                            table::fixed(PARAGRAPH_FONT_SIZE_MEDIUM.into_px(), |wh, ctx| {
                                let range = tower.attack_range_radius(
                                    tower_upgrades,
                                    game_state.stage_modifiers.get_range_multiplier(),
                                );
                                ctx.add(
                                    paragraph(format!("사정거리: {range:.1}"))
                                        .size(FontSize::Medium)
                                        .align(TextAlign::LeftTop)
                                        .max_width(wh.width)
                                        .build(),
                                );
                            }),
                            table::ratio(1.0, |_wh, _ctx| {}),
                            table::fixed(36.px(), |wh, ctx| {
                                let tower_id = tower.id();
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &move || {
                                            mutate_game_state(move |game_state| {
                                                game_state.towers.remove_tower(tower_id);
                                                game_state.selected_tower_id = None;
                                            });
                                        },
                                        &|wh, text_color, ctx| {
                                            ctx.add(
                                                paragraph("철거".to_string())
                                                    .size(FontSize::Medium)
                                                    .align(TextAlign::Center { wh })
                                                    .color(text_color)
                                                    .max_width(wh.width)
                                                    .build(),
                                            );
                                        },
                                    )
                                    .variant(ButtonVariant::Contained)
                                    .color(ButtonColor::Error),
                                );
                            }),
                        ])(wh, ctx);
                    })(Wh::new(BUBBLE_WIDTH, BUBBLE_HEIGHT), ctx);
                });

                ctx.add(rect(RectParam {
                    rect: Wh::new(BUBBLE_WIDTH, BUBBLE_HEIGHT).to_rect(),
                    style: RectStyle {
                        fill: Some(RectFill {
                            color: palette::SURFACE_CONTAINER_HIGHEST,
                        }),
                        stroke: Some(RectStroke {
                            color: palette::OUTLINE,
                            width: 1.px(),
                            border_position: BorderPosition::Inside,
                        }),
                        round: Some(RectRound {
                            radius: palette::ROUND,
                        }),
                    },
                }));
            })
            .attach_event(|event| {
                if let Event::MouseUp { event } = event
                    && let Some(MouseButton::Left) = event.button
                    && event.is_local_xy_in()
                {
                    event.stop_propagation();
                }
            });
    }
}
