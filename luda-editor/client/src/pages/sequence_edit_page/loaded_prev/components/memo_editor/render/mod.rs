use super::*;
use crate::color;
use namui::text_input::Style;
use namui_prebuilt::{
    button::text_button_fit,
    simple_rect,
    table::{self, TableCell},
};

impl MemoEditor {
    pub fn render(&self, props: Props) -> RenderingTree {
        const PADDING: Px = px(8.0);
        const TITLE_HEIGHT: Px = px(48.0);

        let screen_wh = screen::size();

        on_top(render([
            render_background(screen_wh),
            translate(
                (screen_wh.width - props.wh.width) / 2.0,
                (screen_wh.height - props.wh.height) / 2.0,
                render([
                    render_container(props.wh),
                    table::vertical([
                        table::fixed(TITLE_HEIGHT, |wh| {
                            render([
                                simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT),
                                table::padding(PADDING, |wh: Wh<Px>| {
                                    table::horizontal([
                                        render_cancel_button(wh.height, PADDING),
                                        table::ratio(1, |_| RenderingTree::Empty),
                                        render_save_button(wh.height, PADDING),
                                    ])(wh)
                                })(wh),
                            ])
                        }),
                        table::ratio(1, |wh| {
                            render([
                                simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT),
                                table::padding(PADDING, |wh| self.render_text_input(wh, PADDING))(
                                    wh,
                                ),
                            ])
                        }),
                    ])(props.wh),
                ]),
            ),
        ]))
    }

    fn render_text_input(&self, wh: Wh<Px>, padding: Px) -> RenderingTree {
        self.text_input.render(text_input::Props {
            rect: Rect::from_xy_wh(Xy::zero(), wh),
            text: self.text.to_string(),
            text_align: TextAlign::Left,
            text_baseline: TextBaseline::Top,
            font_type: FontType {
                serif: false,
                size: 14.int_px(),
                language: Language::Ko,
                font_weight: FontWeight::REGULAR,
            },
            style: Style {
                padding: Ltrb {
                    left: padding,
                    top: padding,
                    right: padding,
                    bottom: padding,
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
            event_handler: Some(text_input::EventHandler::new().on_text_updated(
                move |text: String| namui::event::send(InternalEvent::ChangeText(text)),
            )),
        })
    }
}

fn render_background(wh: Wh<Px>) -> RenderingTree {
    simple_rect(
        wh,
        Color::TRANSPARENT,
        0.px(),
        Color::grayscale_alpha_f01(0.0, 0.5),
    )
    .attach_event(|builder| {
        builder.on_mouse_down_in(|event: MouseEvent| {
            event.stop_propagation();
            namui::event::send(Event::CloseMemoEditor)
        });
    })
    .with_mouse_cursor(MouseCursor::Default)
}

fn render_container(wh: Wh<Px>) -> RenderingTree {
    simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND).attach_event(|builder| {
        builder.on_mouse_down_in(|event: MouseEvent| {
            event.stop_propagation();
        });
    })
}

fn render_cancel_button<'a>(height: Px, padding: Px) -> TableCell<'a> {
    table::fit(
        table::FitAlign::LeftTop,
        text_button_fit(
            height,
            "취소",
            color::STROKE_NORMAL,
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
            padding,
            [MouseButton::Left],
            |_event| {
                namui::event::send(Event::CloseMemoEditor);
            },
        )
        .with_mouse_cursor(MouseCursor::Pointer),
    )
}

fn render_save_button<'a>(height: Px, padding: Px) -> TableCell<'a> {
    table::fit(
        table::FitAlign::RightBottom,
        text_button_fit(
            height,
            "저장",
            color::BACKGROUND,
            color::STROKE_NORMAL,
            1.px(),
            color::STROKE_NORMAL,
            padding,
            [MouseButton::Left],
            |_event| {
                namui::event::send(InternalEvent::SaveButtonClicked);
            },
        )
        .with_mouse_cursor(MouseCursor::Pointer),
    )
}
