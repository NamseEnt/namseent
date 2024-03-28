use namui::*;

pub fn main() {
    namui::start(|| TextInputExample::new())
}

#[namui::component]
struct TextInputExample {}

impl TextInputExample {
    fn new() -> Self {
        Self {}
    }
}

impl Component for TextInputExample {
    fn render<'a>(self, ctx: &'a RenderCtx)  {
        let (text_3x3, set_text_3x3) = ctx.state(|| {
            [
                [
                    "Left Top\nHel🔗lo you!\nmamama mimimi mo".to_string(),
                    "Center Top\nHello yo🔗u!\nmamama mimimi mo".to_string(),
                    "Right Top\nHello you!\nmamama mimimi mo".to_string(),
                ],
                [
                    "Left Center\nHello you!\nmamama mimimi mo".to_string(),
                    "Center Center\nHello you!🔗\nmamama mimimi mo".to_string(),
                    "Right Center\nHe🔗llo you!\nmamama mimimi mo".to_string(),
                ],
                [
                    "Left Bottom\nHello you!\nmama🔗ma mimimi mo".to_string(),
                    "Center Bottom\n🔗Hello you!\nmamama mimimi mo".to_string(),
                    "Right Bottom\nHell🔗o you!\nmamama mimimi mo".to_string(),
                ],
            ]
        });
        let text_input_instances_3x3 = [
            [
                TextInputInstance::new(ctx),
                TextInputInstance::new(ctx),
                TextInputInstance::new(ctx),
            ],
            [
                TextInputInstance::new(ctx),
                TextInputInstance::new(ctx),
                TextInputInstance::new(ctx),
            ],
            [
                TextInputInstance::new(ctx),
                TextInputInstance::new(ctx),
                TextInputInstance::new(ctx),
            ],
        ];

        ctx.compose(|ctx| {
            for x in 0..3 {
                for y in 0..3 {
                    let key = format!("{}-{}", x, y);
                    ctx.add_with_key(
                        key,
                        text_input::TextInput {
                            instance: text_input_instances_3x3[x][y],
                            rect: Rect::Xywh {
                                x: (x as f32 * 300.0 + 100.0).px(),
                                y: (y as f32 * 300.0 + 100.0).px(),
                                width: px(200.0),
                                height: px(200.0),
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
                            text: (*text_3x3)[x][y].clone(),
                            font: namui::Font {
                                name: "NotoSansKR-Regular".to_string(),
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
                            on_event: boxed(move |event| match event {
                                text_input::Event::TextUpdated { text } => {
                                    let text = text.to_string();
                                    set_text_3x3.mutate(move |text_3x3| {
                                        text_3x3[x][y] = text;
                                    });
                                }
                                _ => {}
                            }),
                            prevent_default_codes: vec![],
                        },
                    );
                }
            }
        });

        
    }

    // fn update(&mut self, event: &namui::Event) {
    //     event.is::<text_input::Event>(|event| {
    //         match event {
    //             text_input::Event::TextUpdated { id, text, .. } => {
    //                 for x in 0..3 {
    //                     for y in 0..3 {
    //                         if self.text_input_3x3[x][y].get_id() == *id {
    //                             self.text_3x3[x][y] = text.clone();

    //                             if x == 0 && y == 0 {
    //                                 self.left_top_value = self.text_3x3[x][y].parse().ok();
    //                                 // NOTE: You don't have to check value in here, it's would be better UX checking it on blur.
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //             text_input::Event::Blur { id } => {
    //                 if self.text_input_3x3[0][0].get_id().eq(id) {
    //                     self.left_top_value = self.text_3x3[0][0].parse().ok();
    //                 }
    //             }
    //             _ => {}
    //         }
    //     });
    // }
}
