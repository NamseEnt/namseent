use namui::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn start() {
    let namui_context = namui::init();

    namui::start(namui_context, &mut TextInputExample::new(), &()).await
}

struct TextInputExample {
    left_text_input: namui::TextInput,
    center_text_input: namui::TextInput,
    right_text_input: namui::TextInput,
    left_text: String,
    center_text: String,
    right_text: String,
}

impl TextInputExample {
    fn new() -> Self {
        Self {
            left_text_input: namui::TextInput::new(),
            center_text_input: namui::TextInput::new(),
            right_text_input: namui::TextInput::new(),
            left_text: "Left".to_string(),
            center_text: "Center".to_string(),
            right_text: "Right".to_string(),
        }
    }
}

impl Entity for TextInputExample {
    type Props = ();

    fn render(&self, props: &Self::Props) -> RenderingTree {
        let left = self.left_text_input.render(namui::text_input::Props {
            rect_param: namui::RectParam {
                x: 200.0,
                y: 200.0,
                width: 200.0,
                height: 200.0,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        border_position: BorderPosition::Inside,
                        color: Color::BLACK,
                        width: 1.0,
                    }),
                    ..Default::default()
                },
            },
            text_param: namui::TextParam {
                x: 200.0,
                y: 200.0,
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                text: self.left_text.clone(),
                font_type: namui::FontType {
                    font_weight: namui::FontWeight::REGULAR,
                    language: namui::Language::Ko,
                    serif: false,
                    size: 20,
                },
                style: namui::TextStyle {
                    color: namui::Color::BLACK,
                    ..Default::default()
                },
            },
        });

        let center = self.center_text_input.render(namui::text_input::Props {
            rect_param: namui::RectParam {
                x: 500.0,
                y: 200.0,
                width: 200.0,
                height: 200.0,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        border_position: BorderPosition::Inside,
                        color: Color::BLACK,
                        width: 1.0,
                    }),
                    ..Default::default()
                },
            },
            text_param: namui::TextParam {
                x: 500.0,
                y: 200.0,
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                text: self.center_text.clone(),
                font_type: namui::FontType {
                    font_weight: namui::FontWeight::REGULAR,
                    language: namui::Language::Ko,
                    serif: false,
                    size: 20,
                },
                style: namui::TextStyle {
                    color: namui::Color::BLACK,
                    ..Default::default()
                },
            },
        });

        let right = self.right_text_input.render(namui::text_input::Props {
            rect_param: namui::RectParam {
                x: 800.0,
                y: 200.0,
                width: 200.0,
                height: 200.0,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        border_position: BorderPosition::Inside,
                        color: Color::BLACK,
                        width: 1.0,
                    }),
                    ..Default::default()
                },
            },
            text_param: namui::TextParam {
                x: 800.0,
                y: 200.0,
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                text: self.right_text.clone(),
                font_type: namui::FontType {
                    font_weight: namui::FontWeight::REGULAR,
                    language: namui::Language::Ko,
                    serif: false,
                    size: 20,
                },
                style: namui::TextStyle {
                    color: namui::Color::BLACK,
                    ..Default::default()
                },
            },
        });

        render![left, center, right]
    }

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<text_input::Event>() {
            match event {
                text_input::Event::TextUpdated(text_updated) => {
                    if self.left_text_input.get_id().eq(&text_updated.id) {
                        self.left_text = text_updated.text.clone();
                    } else if self.center_text_input.get_id().eq(&text_updated.id) {
                        self.center_text = text_updated.text.clone();
                    } else if self.right_text_input.get_id().eq(&text_updated.id) {
                        self.right_text = text_updated.text.clone();
                    }
                }
                _ => {}
            }
        }
        self.left_text_input.update(event);
        self.center_text_input.update(event);
        self.right_text_input.update(event);
    }
}
