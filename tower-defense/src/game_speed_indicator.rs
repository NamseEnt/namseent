use crate::{
    game_state::{fast_forward::FastForwardMultiplier, mutate_game_state, use_game_state},
    theme::typography,
    theme::{button::Button, palette},
};
use namui::*;
use namui_prebuilt::table;

pub struct GameSpeedIndicator;

impl Component for GameSpeedIndicator {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);

        let current_speed_text = match game_state.fast_forward_multiplier {
            FastForwardMultiplier::X1 => "1x",
            FastForwardMultiplier::X2 => "2x",
            FastForwardMultiplier::X4 => "4x",
            FastForwardMultiplier::X8 => "8x",
            FastForwardMultiplier::X16 => "16x",
        };

        let slower_action = || {
            mutate_game_state(|game_state| {
                game_state.fast_forward_multiplier = game_state.fast_forward_multiplier.prev();
            });
        };

        let default_action = || {
            mutate_game_state(|game_state| {
                game_state.fast_forward_multiplier = FastForwardMultiplier::X1;
            });
        };

        let faster_action = || {
            mutate_game_state(|game_state| {
                game_state.fast_forward_multiplier = game_state.fast_forward_multiplier.next();
            });
        };

        const PADDING: Px = px(8.);
        let content_wh = Wh::new(92.px(), 64.px());
        let container_wh = Wh::new(
            content_wh.width + (PADDING * 2.0),
            content_wh.height + (PADDING * 2.0),
        );

        ctx.translate((PADDING, PADDING)).compose(|ctx| {
            table::vertical([
                // Current speed display
                table::fixed(32.px(), |wh, ctx| {
                    ctx.add(
                        typography::paragraph()
                            .text(&format!("Speed: {current_speed_text}"))
                            .center(wh),
                    );
                }),
                // Buttons row
                table::fixed(28.px(), |_wh, ctx| {
                    table::horizontal([
                        // Slower button
                        table::fixed(28.px(), |wh, ctx| {
                            ctx.add(
                                Button::new(wh, &slower_action, &|wh, color, ctx| {
                                    ctx.add(
                                        typography::paragraph()
                                            .text("<<")
                                            .color(color)
                                            .center(wh),
                                    );
                                })
                                .disabled(
                                    game_state.fast_forward_multiplier == FastForwardMultiplier::X1,
                                ),
                            );
                        }),
                        // Default button
                        table::fixed(36.px(), |wh, ctx| {
                            ctx.add(Button::new(wh, &default_action, &|wh, color, ctx| {
                                ctx.add(
                                    typography::paragraph()
                                        .text("1x")
                                        .color(color)
                                        .center(wh),
                                );
                            }));
                        }),
                        // Faster button
                        table::fixed(28.px(), |wh, ctx| {
                            ctx.add(
                                Button::new(wh, &faster_action, &|wh, color, ctx| {
                                    ctx.add(
                                        typography::paragraph()
                                            .text(">>")
                                            .color(color)
                                            .center(wh),
                                    );
                                })
                                .disabled(
                                    game_state.fast_forward_multiplier
                                        == FastForwardMultiplier::X16,
                                ),
                            );
                        }),
                    ])(Wh::new(92.px(), 28.px()), ctx);
                }),
            ])(content_wh, ctx);
        });

        // Background for entire component
        ctx.add(rect(RectParam {
            rect: container_wh.to_rect(),
            style: RectStyle {
                fill: Some(RectFill {
                    color: palette::SURFACE_CONTAINER,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
            },
        }));
    }
}
