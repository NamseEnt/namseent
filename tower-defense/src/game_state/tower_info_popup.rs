use super::{Tower, mutate_game_state, upgrade::TowerUpgradeState};
use crate::theme::{
    button::{Button, ButtonColor, ButtonVariant},
    palette,
    typography::{FontSize, TextAlign, paragraph},
};
use crate::tower_display::tower_stat_display_from_tower;
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
                            // 타워 스탯 표시 영역
                            table::ratio(1.0, |wh, ctx| {
                                ctx.add(tower_stat_display_from_tower(
                                    tower,
                                    tower_upgrades,
                                    game_state,
                                    wh,
                                ));
                            }),
                            // 철거 버튼
                            table::fixed(36.px(), |wh, ctx| {
                                let tower_id = tower.id();
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &move || {
                                            mutate_game_state(move |game_state| {
                                                game_state.towers.remove_tower(tower_id);
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

                // 배경 및 테두리
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
