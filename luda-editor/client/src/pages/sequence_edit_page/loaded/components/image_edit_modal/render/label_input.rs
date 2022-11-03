use super::*;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageEditModal {
    pub fn render_label_input(&self, props: Props) -> namui::RenderingTree {
        table::vertical([
            table::fit(
                table::FitAlign::LeftTop,
                typography::title::left_top("Input Label", Color::WHITE).padding(12.px()),
            ),
            table::fixed(
                typography::body::FONT_SIZE.into_px() + 16.px(),
                table::padding(
                    4.px(),
                    table::horizontal([
                        table::fit(
                            table::FitAlign::CenterMiddle,
                            typography::body::left_top("label: ", Color::WHITE),
                        ),
                        table::ratio(1, |wh| {
                            self.label_text_input.render(text_input::Props {
                                rect: Rect::from_xy_wh(Xy::zero(), wh),
                                text: self.label_text.clone(),
                                text_align: TextAlign::Left,
                                text_baseline: TextBaseline::Middle,
                                font_type: FontType {
                                    serif: false,
                                    size: typography::body::FONT_SIZE,
                                    language: Language::Ko,
                                    font_weight: FontWeight::REGULAR,
                                },
                                style: text_input::Style {
                                    rect: RectStyle {
                                        stroke: Some(RectStroke {
                                            color: Color::WHITE,
                                            width: 1.px(),
                                            border_position: BorderPosition::Inside,
                                        }),
                                        fill: Some(RectFill {
                                            color: Color::BLACK,
                                        }),
                                        ..Default::default()
                                    },
                                    text: TextStyle {
                                        color: Color::WHITE,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                event_handler: Some(text_input::EventHandler::new().on_key_down(
                                    |event| {
                                        if event.code == Code::Enter && !event.is_composing {
                                            namui::event::send(
                                                InternalEvent::LabelInputEnterPressed,
                                            );
                                            event.prevent_default();
                                        }
                                    },
                                )),
                            })
                        }),
                    ]),
                ),
            ),
        ])(props.wh)
    }
}
