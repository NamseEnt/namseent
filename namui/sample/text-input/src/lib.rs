use namui::prelude::*;

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut TextInputExample::new(), &()).await
}

struct TextInputExample {
    left_text_input: namui::TextInput,
    center_text_input: namui::TextInput,
    right_text_input: namui::TextInput,
    left_text: String,
    center_text: String,
    right_text: String,
    left_value: Option<f32>,
}

impl TextInputExample {
    fn new() -> Self {
        Self {
            left_text_input: namui::TextInput::new(),
            center_text_input: namui::TextInput::new(),
            right_text_input: namui::TextInput::new(),
            left_text: "Left\nHelloy\n    SameText".to_string(),
            center_text: "Center\nworldy\nSameText".to_string(),
            right_text: "Right\n안녕하세요.\nSameText    ".to_string(),
            left_value: None,
        }
    }
}

impl Entity for TextInputExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let left = self.left_text_input.render(namui::text_input::Props {
            rect: Rect::Xywh {
                x: 200.0.px(),
                y: px(200.0),
                width: px(200.0),
                height: px(200.0),
            },
            rect_style: RectStyle {
                stroke: Some(RectStroke {
                    border_position: BorderPosition::Inside,
                    color: Color::BLACK,
                    width: px(1.0),
                }),
                ..Default::default()
            },
            text_align: TextAlign::Left,
            text_baseline: TextBaseline::Top,
            text: self.left_text.clone(),
            font_type: namui::FontType {
                font_weight: namui::FontWeight::REGULAR,
                language: namui::Language::Ko,
                serif: false,
                size: int_px(20),
            },
            text_style: namui::TextStyle {
                color: namui::Color::BLACK,
                ..Default::default()
            },
            event_handler: None,
        });

        let center = self.center_text_input.render(namui::text_input::Props {
            rect: Rect::Xywh {
                x: px(500.0),
                y: px(200.0),
                width: px(200.0),
                height: px(200.0),
            },
            rect_style: RectStyle {
                stroke: Some(RectStroke {
                    border_position: BorderPosition::Inside,
                    color: Color::BLACK,
                    width: px(1.0),
                }),
                ..Default::default()
            },
            text_align: TextAlign::Center,
            text_baseline: TextBaseline::Top,
            text: self.center_text.clone(),
            font_type: namui::FontType {
                font_weight: namui::FontWeight::REGULAR,
                language: namui::Language::Ko,
                serif: false,
                size: int_px(20),
            },
            text_style: namui::TextStyle {
                color: namui::Color::BLACK,
                ..Default::default()
            },
            event_handler: None,
        });

        let right = self.right_text_input.render(namui::text_input::Props {
            rect: Rect::Xywh {
                x: px(800.0),
                y: px(200.0),
                width: px(200.0),
                height: px(200.0),
            },
            rect_style: RectStyle {
                stroke: Some(RectStroke {
                    border_position: BorderPosition::Inside,
                    color: Color::BLACK,
                    width: px(1.0),
                }),
                ..Default::default()
            },
            text_align: TextAlign::Right,
            text_baseline: TextBaseline::Top,
            text: self.right_text.clone(),
            font_type: namui::FontType {
                font_weight: namui::FontWeight::REGULAR,
                language: namui::Language::Ko,
                serif: false,
                size: int_px(20),
            },
            text_style: namui::TextStyle {
                color: namui::Color::BLACK,
                ..Default::default()
            },
            event_handler: Some(text_input::EventHandler::new().on_key_down(|event| {
                if event.code == namui::Code::Digit7 {
                    namui::log!("7 key is pressed, prevent default");
                    event.prevent_default();
                }
            })),
        });

        let left_value_text = namui::text(TextParam {
            x: px(200.0),
            y: px(500.0),
            align: TextAlign::Left,
            baseline: TextBaseline::Top,
            text: self
                .left_value
                .map(|v| v.to_string())
                .unwrap_or("is't not f32".to_string()),
            font_type: namui::FontType {
                font_weight: namui::FontWeight::REGULAR,
                language: namui::Language::Ko,
                serif: false,
                size: int_px(20),
            },
            style: namui::TextStyle {
                color: namui::Color::BLACK,
                ..Default::default()
            },
            max_width: Some(100.px()),
        });

        render![left, center, right, left_value_text]
    }

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<text_input::Event>() {
            match event {
                text_input::Event::TextUpdated { id, text, .. } => {
                    if self.left_text_input.get_id().eq(id) {
                        self.left_text = text.clone();
                        self.left_value = self.left_text.parse().ok(); // NOTE: You don't have to check value in here, it's would be better UX checking it on blur.
                    } else if self.center_text_input.get_id().eq(id) {
                        self.center_text = text.clone();
                    } else if self.right_text_input.get_id().eq(id) {
                        self.right_text = text.clone();
                    }
                }
                text_input::Event::Blur { id } => {
                    if self.left_text_input.get_id().eq(id) {
                        self.left_value = self.left_text.parse().ok();
                    }
                }
                _ => {}
            }
        }
    }
}
