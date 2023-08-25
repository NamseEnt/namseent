use crate::color;
use namui::{prelude::*, text_input::Style};
use namui_prebuilt::*;

#[namui::component]
pub struct MemoEditor<'a> {
    pub sequence_id: Uuid,
    pub cut_id: Uuid,
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

pub enum Event {
    Close,
    SaveButtonClicked {
        sequence_id: Uuid,
        cut_id: Uuid,
        content: String,
    },
}

impl Component for MemoEditor<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        const MEMO_EDITOR_WH: Wh<Px> = Wh {
            width: px(512.0),
            height: px(256.0),
        };
        let Self {
            sequence_id,
            cut_id,
            ref on_event,
        } = self;

        let (text, set_text) = ctx.state(|| "".to_string());
        let text_input_instance = namui::text_input::TextInputInstance::new(ctx);

        const PADDING: Px = px(8.0);
        const TITLE_HEIGHT: Px = px(48.0);

        let screen_wh = screen::size();
        let screen_wh = Wh::new(screen_wh.width.into_px(), screen_wh.height.into_px());

        let background = simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::grayscale_alpha_f01(0.0, 0.5),
        )
        .with_mouse_cursor(MouseCursor::Default);

        let container = simple_rect(
            MEMO_EDITOR_WH,
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
        )
        .attach_event(|event| match event {
            namui::Event::MouseDown { event } => {
                if !event.is_local_xy_in() {
                    event.stop_propagation();
                    on_event(Event::Close);
                }
            }
            _ => {}
        });

        let render_close_button = |height: Px| {
            table::hooks::fit(
                table::hooks::FitAlign::LeftTop,
                button::text_button_fit(
                    height,
                    "취소",
                    color::STROKE_NORMAL,
                    color::STROKE_NORMAL,
                    1.px(),
                    color::BACKGROUND,
                    PADDING,
                    [MouseButton::Left],
                    {
                        let on_event = on_event.clone();
                        move |_event| {
                            on_event(Event::Close);
                        }
                    },
                )
                .with_mouse_cursor(MouseCursor::Pointer),
            )
        };

        let render_save_button = |height: Px| {
            table::hooks::fit(
                table::hooks::FitAlign::RightBottom,
                button::text_button_fit(
                    height,
                    "저장",
                    color::BACKGROUND,
                    color::STROKE_NORMAL,
                    1.px(),
                    color::STROKE_NORMAL,
                    PADDING,
                    [MouseButton::Left],
                    {
                        let on_event = on_event.clone();
                        |_event| {
                            on_event(Event::SaveButtonClicked {
                                sequence_id,
                                cut_id,
                                content: text.to_string(),
                            });
                        }
                    },
                )
                .with_mouse_cursor(MouseCursor::Pointer),
            )
        };

        let content = table::hooks::vertical([
            table::hooks::fixed(TITLE_HEIGHT, |wh, ctx| {
                ctx.add(simple_rect(
                    wh,
                    color::STROKE_NORMAL,
                    1.px(),
                    Color::TRANSPARENT,
                ));

                table::hooks::padding(PADDING, |wh, ctx| {
                    table::hooks::horizontal([
                        render_close_button(wh.height),
                        table::hooks::ratio(1, |_, _| {}),
                        render_save_button(wh.height),
                    ])(wh, ctx);
                })(wh, ctx);
            }),
            table::hooks::ratio(1, |wh, ctx| {
                ctx.add(simple_rect(
                    wh,
                    color::STROKE_NORMAL,
                    1.px(),
                    Color::TRANSPARENT,
                ));

                table::hooks::padding(PADDING, |wh, ctx| {
                    ctx.add(text_input::TextInput {
                        instance: text_input_instance,
                        rect: Rect::from_xy_wh(Xy::zero(), wh),
                        text: text.to_string(),
                        text_align: TextAlign::Left,
                        text_baseline: TextBaseline::Top,
                        font: Font {
                            size: 14.int_px(),
                            name: "NotoSansKR-Regular".to_string(),
                        },
                        style: Style {
                            // TODO: Declare Ltrb with vector_types! macro
                            // padding: Ltrb::single(PADDING),
                            padding: Ltrb {
                                left: PADDING,
                                top: PADDING,
                                right: PADDING,
                                bottom: PADDING,
                            },
                            rect: RectStyle {
                                stroke: Some(RectStroke {
                                    color: color::STROKE_NORMAL,
                                    width: 1.px(),
                                    border_position: BorderPosition::Inside,
                                }),
                                fill: None,
                                round: None,
                            },
                            text: TextStyle {
                                color: color::STROKE_NORMAL,
                                ..Default::default()
                            },
                        },
                        prevent_default_codes: vec![],
                        on_event: &|event| match event {
                            text_input::Event::TextUpdated { text } => {
                                set_text.set(text.to_string())
                            }
                            _ => {}
                        },
                    });
                })(wh, ctx);
            }),
        ]);

        ctx.compose(|ctx| {
            let mut ctx = ctx.on_top().add(background).translate((
                (screen_wh.width - MEMO_EDITOR_WH.width) / 2.0,
                (screen_wh.height - MEMO_EDITOR_WH.height) / 2.0,
            ));
            ctx.add(container);
            content(MEMO_EDITOR_WH, &mut ctx);
            ctx.add(namui_prebuilt::event_trap::EventTrap);
        })
        .done()
    }
}
