use crate::color;
use namui::prelude::*;
use namui_prebuilt::*;

#[namui::component]
pub struct MemoEditor<'a> {
    pub wh: Wh<Px>,
    pub sequence_id: Uuid,
    pub cut_id: Uuid,
    pub on_event: callback!('a, Event),
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
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            wh,
            sequence_id,
            cut_id,
            ref on_event,
        } = self;

        let (text, set_text) = ctx.state(|| "".to_string());

        const PADDING: Px = px(8.0);
        const TITLE_HEIGHT: Px = px(48.0);

        let screen_wh = screen::size();

        let background = {
            simple_rect(
                screen_wh,
                Color::TRANSPARENT,
                0.px(),
                Color::grayscale_alpha_f01(0.0, 0.5),
            )
            .attach_event(|builder| {
                let on_event = on_event.clone();
                builder.on_mouse_down_in(move |event: MouseEvent| {
                    event.stop_propagation();
                    on_event(Event::Close);
                });
            })
            .with_mouse_cursor(MouseCursor::Default)
        };

        let container = simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND)
            .attach_event(|builder| {
                builder.on_mouse_down_in(|event: MouseEvent| {
                    event.stop_propagation();
                });
            });

        let close_button_cell = move |wh: Wh<Px>| {
            table::hooks::fit(
                table::hooks::FitAlign::LeftTop,
                button::text_button_fit(
                    wh.height,
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

        // let on_save_button_clicked = || {
        //     let content = text.clone();
        //     spawn_local(async move {
        //         match crate::RPC
        //             .create_memo(rpc::create_memo::Request {
        //                 sequence_id,
        //                 cut_id,
        //                 content,
        //             })
        //             .await
        //         {
        //             Ok(response) => {
        //                 namui::event::send(Event::MemoCreated {
        //                     memo: response.memo,
        //                 });
        //                 namui::event::send(Event::CloseMemoEditor);
        //             }
        //             Err(error) => namui::log!("Failed to create memo: {:?}", error),
        //         };
        //     });
        // };

        let save_button_cell = move |wh: Wh<Px>| {
            table::hooks::fit(
                table::hooks::FitAlign::RightBottom,
                button::text_button_fit(
                    wh.height,
                    "저장",
                    color::BACKGROUND,
                    color::STROKE_NORMAL,
                    1.px(),
                    color::STROKE_NORMAL,
                    PADDING,
                    [MouseButton::Left],
                    {
                        let on_event = on_event.clone();
                        move |_event| {
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
            table::hooks::fixed(TITLE_HEIGHT, |wh| {
                (
                    simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT),
                    table::hooks::padding(PADDING, |wh: Wh<Px>| {
                        table::hooks::horizontal([
                            close_button_cell(wh),
                            table::hooks::ratio(1, |_| RenderingTree::Empty),
                            save_button_cell(wh),
                        ])(wh)
                    })(wh),
                )
            }),
            table::hooks::ratio(1, |wh| {
                (
                    simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT),
                    // table::hooks::padding(PADDING, |wh| text_input::TextInput {
                    //     rect: Rect::from_xy_wh(Xy::zero(), wh),
                    //     text: text.to_string(),
                    //     text_align: TextAlign::Left,
                    //     text_baseline: TextBaseline::Top,
                    //     font_type: FontType {
                    //         serif: false,
                    //         size: 14.int_px(),
                    //         language: Language::Ko,
                    //         font_weight: FontWeight::REGULAR,
                    //     },
                    //     style: Style {
                    //         // TODO: Declare Ltrb with vector_types! macro
                    //         // padding: Ltrb::single(PADDING),
                    //         padding: Ltrb {
                    //             left: PADDING,
                    //             top: PADDING,
                    //             right: PADDING,
                    //             bottom: PADDING,
                    //         },
                    //         rect: RectStyle {
                    //             stroke: Some(RectStroke {
                    //                 color: color::STROKE_NORMAL,
                    //                 width: 1.px(),
                    //                 border_position: BorderPosition::Inside,
                    //             }),
                    //             fill: None,
                    //             round: None,
                    //         },
                    //         text: TextStyle {
                    //             color: color::STROKE_NORMAL,
                    //             ..Default::default()
                    //         },
                    //     },
                    //     event_handler: Some(
                    //         text_input::EventHandler::new()
                    //             .on_text_updated(move |text| set_text.set(text.clone())),
                    //     ),
                    // })(wh),
                )
            }),
        ])(wh);

        ctx.add(hooks::on_top((
            background,
            hooks::translate(
                (screen_wh.width - wh.width) / 2.0,
                (screen_wh.height - wh.height) / 2.0,
                (container, content),
            ),
        )));
    }
}
