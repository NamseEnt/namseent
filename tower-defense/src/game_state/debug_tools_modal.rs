use crate::game_state::set_modal;
use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::{
    palette,
    typography::{self, headline},
};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(16.);

pub struct DebugToolsModal;

impl Component for DebugToolsModal {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();

        let modal_wh = Wh::new(600.px(), 400.px());
        let modal_xy = ((screen_wh - modal_wh) * 0.5).to_xy();

        ctx.compose(|ctx| {
            // 모달 창
            let ctx = ctx.translate(modal_xy);
            ctx.compose(|ctx| {
                table::vertical([
                    table::fixed(
                        TITLE_HEIGHT,
                        table::horizontal([
                            table::fixed(PADDING, |_, _| {}),
                            table::ratio(1, |wh, ctx| {
                                ctx.add(
                                    headline("Debug Tools")
                                        .size(typography::FontSize::Medium)
                                        .align(typography::TextAlign::LeftCenter { height: wh.height })
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
                    table::ratio(
                        1,
                        table::padding(PADDING, |wh, ctx| {
                            // Empty content for now
                            ctx.add(simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT));
                        }),
                    ),
                ])(modal_wh, ctx);
            });

            // 타이틀 배경
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

            // 모달 배경
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

        // 배경 오버레이
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
