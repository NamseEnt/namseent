use crate::game_state::set_modal;
use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::{
    button::{Button, ButtonVariant},
    palette,
    typography::{self, headline},
};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(16.);

pub struct ChallengeModal;

impl Component for ChallengeModal {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();

        let modal_wh = Wh::new(320.px(), 180.px());
        let modal_xy = ((screen_wh - modal_wh) * 0.5).to_xy();

        ctx.compose(|ctx| {
            // Modal container
            let ctx = ctx.translate(modal_xy);
            ctx.compose(|ctx| {
                table::vertical([
                    table::fixed(
                        TITLE_HEIGHT,
                        table::horizontal([
                            table::fixed(PADDING, |_, _| {}),
                            table::ratio(1, |wh, ctx| {
                                ctx.add(
                                    headline("도전")
                                        .size(typography::FontSize::Medium)
                                        .align(typography::TextAlign::LeftCenter {
                                            height: wh.height,
                                        })
                                        .build(),
                                );
                            }),
                            table::fixed(48.px(), |wh, ctx| {
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &|| set_modal(None),
                                        &|wh, _text_color, ctx| {
                                            ctx.add(
                                                Icon::new(IconKind::Reject)
                                                    .size(IconSize::Large)
                                                    .wh(wh),
                                            );
                                        },
                                    )
                                    .variant(ButtonVariant::Text),
                                );
                            }),
                        ]),
                    ),
                    table::ratio(1, table::padding(PADDING, |_, _| {})),
                ])(modal_wh, ctx);
            });

            // Title background
            ctx.add(rect(RectParam {
                rect: Wh::new(modal_wh.width, TITLE_HEIGHT).to_rect(),
                style: RectStyle {
                    stroke: None,
                    fill: Some(RectFill {
                        color: palette::SURFACE_CONTAINER,
                    }),
                    round: Some(RectRound {
                        radius: palette::ROUND,
                    }),
                },
            }));

            // Modal background
            ctx.add(rect(RectParam {
                rect: modal_wh.to_rect(),
                style: RectStyle {
                    stroke: None,
                    fill: Some(RectFill {
                        color: palette::SURFACE,
                    }),
                    round: Some(RectRound {
                        radius: palette::ROUND,
                    }),
                },
            }));
        })
        .attach_event(|event| {
            match event {
                Event::MouseDown { event }
                | Event::MouseMove { event }
                | Event::MouseUp { event } => {
                    if !event.is_local_xy_in() {
                        return;
                    }
                    event.stop_propagation();
                }
                Event::Wheel { event } => {
                    if !event.is_local_xy_in() {
                        return;
                    }
                    event.stop_propagation();
                }
                _ => {}
            };
        });

        // Overlay background closes modal on click
        ctx.add(
            simple_rect(
                screen_wh,
                Color::TRANSPARENT,
                0.px(),
                Color::from_u8(0, 0, 0, 128),
            )
            .attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                set_modal(None);
                event.stop_propagation();
            }),
        );
    }
}
