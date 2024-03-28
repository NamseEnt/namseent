use namui::*;

pub fn main() {
    namui::start(&mut TabToNextTextInputExample::new(), &()).await
}

struct TabToNextTextInputExample {
    text_inputs: [(namui::TextInput, String); 3],
}

impl TabToNextTextInputExample {
    fn new() -> Self {
        Self {
            text_inputs: [
                (namui::TextInput::new(), "First Text Input".to_string()),
                (namui::TextInput::new(), "Second Text Input".to_string()),
                (namui::TextInput::new(), "Third Text Input".to_string()),
            ],
        }
    }
}

enum Event {
    TabPress { index: usize, shift: bool },
}

impl Entity for TabToNextTextInputExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let mut tree = vec![];
        for (index, (text_input, text)) in self.text_inputs.iter().enumerate() {
            tree.push(text_input.render(text_input::Props {
                rect: Rect::Xywh {
                    x: 100.0.px(),
                    y: (index as f32 * 300.0 + 100.0).px(),
                    width: px(200.0),
                    height: px(200.0),
                },
                text_align: TextAlign::Left,
                text_baseline: TextBaseline::Top,
                text: text.clone(),
                font: namui::Font {
                    font_weight: namui::FontWeight::REGULAR,
                    language: namui::Language::Ko,
                    serif: false,
                    size: int_px(20),
                },
                style: text_input::Style {
                    rect: RectStyle {
                        stroke: Some(RectStroke {
                            border_position: BorderPosition::Inside,
                            color: Color::BLACK,
                            width: px(1.0),
                        }),
                        ..Default::default()
                    },
                    text: namui::TextStyle {
                        color: namui::Color::BLACK,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                event_handler: Some(text_input::EventHandler::new().on_key_down(
                    move |event: KeyDownEvent| {
                        if event.code == Code::Tab {
                            event.prevent_default();
                            let shift = namui::keyboard::any_code_press([
                                Code::ShiftLeft,
                                Code::ShiftRight,
                            ]);
                            namui::event::send(Event::TabPress { index, shift });
                        }
                    },
                )),
            }));
        }

        render(tree)
    }

    fn update(&mut self, event: &namui::Event) {
        event
            .is::<text_input::Event>(|event| match event {
                text_input::Event::TextUpdated { id, text, .. } => {
                    self.text_inputs.iter_mut().for_each(|(text_input, _text)| {
                        if text_input.get_id().eq(id) {
                            *_text = text.clone();
                        }
                    });
                }
                _ => {}
            })
            .is::<Event>(|event| match event {
                &Event::TabPress { index, shift } => {
                    let next_index = if shift {
                        if index == 0 {
                            self.text_inputs.len() - 1
                        } else {
                            index - 1
                        }
                    } else if index == self.text_inputs.len() - 1 {
                        0
                    } else {
                        index + 1
                    };

                    self.text_inputs[next_index].0.focus();
                }
            });
    }
}
