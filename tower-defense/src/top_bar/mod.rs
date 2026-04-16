mod game_speed_indicator;
mod run;

use crate::game_state::{Modal, set_modal, use_game_state};
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::top_bar::game_speed_indicator::GameSpeedIndicator;
use crate::top_bar::run::RunIndicator;
use crate::{
    icon::{Icon, IconKind, IconSize},
    palette,
    theme::button::{Button, ButtonVariant},
    theme::typography::{FontSize, memoized_text},
};
use namui::*;
use namui_prebuilt::table;

const TOP_BAR_HEIGHT: Px = px(48.);
const ITEM_WIDTH: Px = px(144.);
const PADDING: Px = px(8.);

const SETTINGS_BUTTON_SIZE: Px = px(36.);
const SPEED_INDICATOR_WIDTH: Px = px(192.);

const BG_OVERSIZE_H: Px = px(4.);
const BG_OVERSIZE_V: Px = px(4.);

pub struct TopBar {
    pub wh: Wh<Px>,
}
impl Component for TopBar {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed_no_clip(ITEM_WIDTH, |wh, ctx| {
                    ctx.add(RunIndicator {
                        wh,
                        stage: game_state.stage,
                    });
                }),
                table::fixed_no_clip(PADDING, |_, _| {}),
                table::fixed_no_clip(ITEM_WIDTH, |wh, ctx| {
                    ctx.compose(|ctx| {
                        let hp_pct = (game_state.hp / 100.0).clamp(0.0, 1.0);
                        let shield = game_state.shield;
                        let has_shield = shield > 0.0;
                        table::horizontal([
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Health).size(IconSize::Large).wh(wh));
                            }),
                            table::fixed_no_clip(72.px(), |wh, ctx| {
                                ctx.add(memoized_text(&(hp_pct, shield), move |mut builder| {
                                    builder
                                        .headline()
                                        .size(FontSize::Medium)
                                        .text(format!("{:.0}", hp_pct * 100.0));

                                    if has_shield {
                                        builder
                                            .text(" (")
                                            .icon(IconKind::Shield)
                                            .text(format!("{:.0}", shield))
                                            .text(")");
                                    }

                                    builder.render_left_center(wh.height)
                                }));
                            }),
                            table::ratio(1, |_, _| {}),
                        ])(wh, ctx);
                    });
                }),
                table::fixed_no_clip(PADDING, |_, _| {}),
                table::fixed_no_clip(ITEM_WIDTH, |wh, ctx| {
                    ctx.compose(|ctx| {
                        table::horizontal([
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Gold).size(IconSize::Large).wh(wh));
                            }),
                            table::fixed_no_clip(32.px(), |wh, ctx| {
                                ctx.add(memoized_text(&game_state.gold, |mut builder| {
                                    builder
                                        .headline()
                                        .size(FontSize::Medium)
                                        .text(format!("{}", game_state.gold))
                                        .render_center(wh)
                                }));
                            }),
                            table::ratio(1, |_, _| {}),
                        ])(wh, ctx);
                    });
                }),
                table::fixed_no_clip(ITEM_WIDTH, |wh, ctx| {
                    ctx.compose(|ctx| {
                        table::horizontal([
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Refresh).size(IconSize::Large).wh(wh));
                            }),
                            table::fixed_no_clip(32.px(), |wh, ctx| {
                                let this_max = game_state.max_dice_chance();
                                ctx.add(memoized_text(
                                    &(game_state.left_dice, this_max),
                                    |mut builder| {
                                        builder
                                            .headline()
                                            .size(FontSize::Medium)
                                            .text(format!("{}/{}", game_state.left_dice, this_max))
                                            .render_center(wh)
                                    },
                                ));
                            }),
                            table::ratio(1, |_, _| {}),
                        ])(wh, ctx);
                    });
                }),
                table::ratio(1, |_, _| {}),
                table::fixed_no_clip(
                    SPEED_INDICATOR_WIDTH,
                    table::padding_no_clip(PADDING, |wh, ctx| {
                        ctx.add(GameSpeedIndicator { wh });
                    }),
                ),
                table::fixed_no_clip(SETTINGS_BUTTON_SIZE + PADDING * 2.0, |wh, ctx| {
                    ctx.translate((PADDING, (wh.height - SETTINGS_BUTTON_SIZE) / 2.0))
                        .add(
                            Button::new(
                                Wh::new(SETTINGS_BUTTON_SIZE, SETTINGS_BUTTON_SIZE),
                                &|| set_modal(Some(Modal::Settings)),
                                &|wh, _text_color, ctx| {
                                    ctx.add(
                                        Icon::new(IconKind::Config).size(IconSize::Large).wh(wh),
                                    );
                                },
                            )
                            .variant(ButtonVariant::Text),
                        );
                }),
            ])(Wh::new(wh.width, TOP_BAR_HEIGHT), ctx);
        });

        ctx.translate((-BG_OVERSIZE_H, -BG_OVERSIZE_V))
            .mouse_cursor(MouseCursor::Standard(StandardCursor::Default))
            .add(PaperContainerBackground {
                width: wh.width + BG_OVERSIZE_H * 2.0,
                height: TOP_BAR_HEIGHT + BG_OVERSIZE_V * 2.0,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Sticky,
                color: palette::SURFACE_CONTAINER,
                shadow: true,
                arrow: None,
            })
            .attach_event(|event| match event {
                Event::MouseDown { event }
                | Event::MouseUp { event }
                | Event::MouseMove { event }
                    if event.is_local_xy_in() =>
                {
                    event.stop_propagation();
                }
                Event::Wheel { event } if event.is_local_xy_in() => {
                    event.stop_propagation();
                }
                _ => {}
            });
    }
}
