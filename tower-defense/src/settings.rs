use crate::icon::{Icon, IconKind, IconSize};
use crate::l10n::ui::TopBarText;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::{
    palette,
    typography::{self, headline},
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(8.);

pub struct SettingsModal<'a> {
    pub screen_wh: Wh<Px>,
    pub close_modal: &'a dyn Fn(),
}

impl Component for SettingsModal<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            close_modal,
        } = self;

        let game_state = crate::game_state::use_game_state(ctx);
        let modal_wh = Wh::new(400.px(), 300.px());
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
                                    headline(
                                        game_state.text().ui(TopBarText::Settings).to_string(),
                                    )
                                    .size(typography::FontSize::Medium)
                                    .align(typography::TextAlign::LeftCenter { height: wh.height })
                                    .build(),
                                );
                            }),
                            table::fixed(64.px(), |wh, ctx| {
                                ctx.add(
                                    Button::new(wh, &|| close_modal(), &|wh, _text_color, ctx| {
                                        ctx.add(
                                            Icon::new(IconKind::Reject)
                                                .size(IconSize::Large)
                                                .wh(wh),
                                        );
                                    })
                                    .variant(ButtonVariant::Text),
                                );
                            }),
                        ]),
                    ),
                    table::ratio(
                        1,
                        table::padding(PADDING, |wh, ctx| {
                            ctx.add(AutoScrollViewWithCtx {
                                wh,
                                scroll_bar_width: PADDING,
                                content: |_ctx| {
                                    // TODO: Add settings content here
                                },
                            });
                        }),
                    ),
                ])(modal_wh, ctx);
            });

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
                close_modal();
                event.stop_propagation();
            }),
        );
    }
}
