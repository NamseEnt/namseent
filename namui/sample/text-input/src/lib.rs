use namui::prelude::*;

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut TextInputExample::new(), &()).await
}

struct TextInputExample {
    text_input_3x3: [[namui::TextInput; 3]; 3],
    text_3x3: [[String; 3]; 3],
    left_top_value: Option<f32>,
}

impl TextInputExample {
    fn new() -> Self {
        Self {
            text_input_3x3: [
                [
                    namui::TextInput::new(),
                    namui::TextInput::new(),
                    namui::TextInput::new(),
                ],
                [
                    namui::TextInput::new(),
                    namui::TextInput::new(),
                    namui::TextInput::new(),
                ],
                [
                    namui::TextInput::new(),
                    namui::TextInput::new(),
                    namui::TextInput::new(),
                ],
            ],
            text_3x3: [
                [
                    "Left Top\nHello you!\nmamama mimimi mo".to_string(),
                    "Center Top\nHello you!\nmamama mimimi mo".to_string(),
                    "Right Top\nHello you!\nmamama mimimi mo".to_string(),
                ],
                [
                    "Left Center\nHello you!\nmamama mimimi mo".to_string(),
                    "Center Center\nHello you!\nmamama mimimi mo".to_string(),
                    "Right Center\nHello you!\nmamama mimimi mo".to_string(),
                ],
                [
                    "Left Bottom\nHello you!\nmamama mimimi mo".to_string(),
                    "Center Bottom\nHello you!\nmamama mimimi mo".to_string(),
                    "Right Bottom\nHello you!\nmamama mimimi mo".to_string(),
                ],
            ],
            left_top_value: None,
        }
    }
}

impl Entity for TextInputExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let mut tree = vec![];
        for x in 0..3 {
            for y in 0..3 {
                tree.push(self.text_input_3x3[x][y].render(text_input::Props {
                    rect: Rect::Xywh {
                        x: (x as f32 * 300.0 + 100.0).px(),
                        y: (y as f32 * 300.0 + 100.0).px(),
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
                    text_align: match x {
                        x if x == 0 => TextAlign::Left,
                        x if x == 1 => TextAlign::Center,
                        x if x == 2 => TextAlign::Right,
                        _ => unreachable!(),
                    },
                    text_baseline: match y {
                        y if y == 0 => TextBaseline::Top,
                        y if y == 1 => TextBaseline::Middle,
                        y if y == 2 => TextBaseline::Bottom,
                        _ => unreachable!(),
                    },
                    text: self.text_3x3[x][y].clone(),
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
                }));
            }
        }

        let left_top_value_text = namui::text(TextParam {
            x: px(10.0),
            y: px(10.0),
            align: TextAlign::Left,
            baseline: TextBaseline::Top,
            text: self
                .left_top_value
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

        tree.push(left_top_value_text);

        render(tree)
    }

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<text_input::Event>() {
            match event {
                text_input::Event::TextUpdated { id, text, .. } => {
                    for x in 0..3 {
                        for y in 0..3 {
                            if self.text_input_3x3[x][y].get_id() == id {
                                self.text_3x3[x][y] = text.clone();

                                if x == 0 && y == 0 {
                                    self.left_top_value = self.text_3x3[x][y].parse().ok();
                                    // NOTE: You don't have to check value in here, it's would be better UX checking it on blur.
                                }
                            }
                        }
                    }
                }
                text_input::Event::Blur { id } => {
                    if self.text_input_3x3[0][0].get_id().eq(id) {
                        self.left_top_value = self.text_3x3[0][0].parse().ok();
                    }
                }
                _ => {}
            }
        }
    }
}
