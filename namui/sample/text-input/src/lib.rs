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
}

impl TextInputExample {
    fn new() -> Self {
        Self {
            left_text_input: namui::TextInput::new(),
            center_text_input: namui::TextInput::new(),
            right_text_input: namui::TextInput::new(),
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
                text: "Left".to_string(),
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
                text: "Center".to_string(),
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
                text: "Right".to_string(),
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
        self.left_text_input.update(event);
        self.center_text_input.update(event);
        self.right_text_input.update(event);
    }
}
