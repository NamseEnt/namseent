use crate::{
    game_state::{fast_forward::FastForwardMultiplier, mutate_game_state, use_game_state},
    icon::{Icon, IconKind, IconSize},
    theme::{
        button::{Button, ButtonVariant},
        palette,
        paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
    },
};
use namui::*;
use namui_prebuilt::table;

pub struct GameSpeedIndicator {
    pub wh: Wh<Px>,
}

fn play_icon(wh: Wh<Px>, opacity: f32) -> Icon {
    Icon {
        opacity,
        ..Icon::new(IconKind::Play)
            .size(IconSize::Custom { size: wh.width })
            .wh(wh)
    }
}

impl Component for GameSpeedIndicator {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        let game_state = use_game_state(ctx);

        let slower_action = || {
            mutate_game_state(|game_state| {
                game_state.fast_forward_multiplier = game_state.fast_forward_multiplier.prev();
            });
        };

        let faster_action = || {
            mutate_game_state(|game_state| {
                game_state.fast_forward_multiplier = game_state.fast_forward_multiplier.next();
            });
        };

        let lit_count = match game_state.fast_forward_multiplier {
            FastForwardMultiplier::X1 => 1,
            FastForwardMultiplier::X2 => 2,
            FastForwardMultiplier::X4 => 3,
            FastForwardMultiplier::X8 | FastForwardMultiplier::X16 => 4,
        };

        const BUTTON_SIZE: Px = px(28.);
        const ICON_SIZE: Px = px(20.);
        const PADDING: Px = px(6.);
        let icon_overlap = ICON_SIZE * 0.25;

        // Render icons first (so the background is behind them)
        ctx.compose(|ctx| {
            table::padding_no_clip(
                PADDING,
                table::horizontal([
                    // Slower button
                    table::fixed_no_clip(BUTTON_SIZE, |wh, ctx| {
                        ctx.add(
                            Button::new(wh, &slower_action, &|wh, _color, ctx| {
                                let ctx =
                                    ctx.scale(Xy::new(-1.0, 1.0)).translate((-wh.width, 0.px()));

                                let icon_wh = Wh::new(ICON_SIZE, ICON_SIZE);
                                let center_x = (wh.width - icon_wh.width) / 2.0;
                                let center_y = (wh.height - icon_wh.height) / 2.0;

                                ctx.translate((center_x - icon_overlap, center_y))
                                    .add(play_icon(icon_wh, 1.0));
                                ctx.translate((center_x + icon_overlap, center_y))
                                    .add(play_icon(icon_wh, 1.0));
                            })
                            .variant(ButtonVariant::Text)
                            .disabled(
                                game_state.fast_forward_multiplier == FastForwardMultiplier::X1,
                            ),
                        );
                    }),
                    // Speed indicator (4 icons)
                    table::ratio_no_clip(1, |wh, ctx| {
                        let icon_wh = Wh::single(wh.height);

                        table::horizontal([
                            table::ratio_no_clip(1, |_, _| {}),
                            table::fixed_no_clip(icon_wh.width, |wh, ctx| {
                                let opacity = if 1 <= lit_count { 1.0 } else { 0.3 };
                                ctx.add(play_icon(wh, opacity));
                            }),
                            table::fixed_no_clip(icon_wh.width, |wh, ctx| {
                                let opacity = if 2 <= lit_count { 1.0 } else { 0.3 };
                                ctx.add(play_icon(wh, opacity));
                            }),
                            table::fixed_no_clip(icon_wh.width, |wh, ctx| {
                                let opacity = if 3 <= lit_count { 1.0 } else { 0.3 };
                                ctx.add(play_icon(wh, opacity));
                            }),
                            table::fixed_no_clip(icon_wh.width, |wh, ctx| {
                                let opacity = if 4 <= lit_count { 1.0 } else { 0.3 };
                                ctx.add(play_icon(wh, opacity));
                            }),
                            table::ratio_no_clip(1, |_, _| {}),
                        ])(wh, ctx);
                    }),
                    // Faster button
                    table::fixed_no_clip(BUTTON_SIZE, |wh, ctx| {
                        ctx.add(
                            Button::new(wh, &faster_action, &|wh, _color, ctx| {
                                let icon_wh = Wh::new(ICON_SIZE, ICON_SIZE);
                                let center_x = (wh.width - icon_wh.width) / 2.0;
                                let center_y = (wh.height - icon_wh.height) / 2.0;

                                ctx.translate((center_x - icon_overlap, center_y))
                                    .add(play_icon(icon_wh, 1.0));
                                ctx.translate((center_x + icon_overlap, center_y))
                                    .add(play_icon(icon_wh, 1.0));
                            })
                            .variant(ButtonVariant::Text)
                            .disabled(
                                game_state.fast_forward_multiplier == FastForwardMultiplier::X16,
                            ),
                        );
                    }),
                ]),
            )(wh, ctx);
        });

        ctx.add(PaperContainerBackground {
            width: self.wh.width,
            height: self.wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Pill,
            color: palette::SURFACE_CONTAINER_LOWEST,
            shadow: false,
            arrow: None,
        });
    }
}
