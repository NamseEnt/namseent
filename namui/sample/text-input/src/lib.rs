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
                    size: 12,
                },
                style: namui::TextStyle {
                    color: namui::Color::BLACK,
                    ..Default::default()
                },
            },
        });

        render![left,]
    }

    fn update(&mut self, event: &dyn std::any::Any) {
        self.left_text_input.update(event);
        self.center_text_input.update(event);
        self.right_text_input.update(event);
    }
}

/*

"Hello world!",
match text_align {
    TextAlign::Left => 200.0,
    TextAlign::Right => 500.0,
    TextAlign::Center => 800.0,
},
200.0,
200.0,
20.0,
namui::Color::WHITE,
namui::Color::BLACK,
1.0,
text_align,
namui::FontType {
    font_weight: namui::FontWeight::REGULAR,
    language: namui::Language::Ko,
    serif: false,
    size: 12,
},
namui::TextStyle {
    color: namui::Color::BLACK,
    ..Default::default()
}, */
