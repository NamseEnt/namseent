use namui::*;

pub fn main() {
    namui::start(|ctx| {
        ctx.add(TextInputExample);
    })
}

struct TextInputExample;

impl Component for TextInputExample {
    fn render(self, ctx: &RenderCtx) {
        let (text_3x3, set_text_3x3) = ctx.state(|| {
            [
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
            ]
        });

        ctx.compose(|ctx| {
            for x in 0..3 {
                for y in 0..3 {
                    let key = format!("{}-{}", x, y);
                    ctx.add_with_key(
                        key,
                        text_input::TextInput {
                            start_text: text_3x3.get(x).unwrap().get(y).unwrap(),
                            rect: Rect::Xywh {
                                x: (x as f32 * 300.0 + 100.0).px(),
                                y: (y as f32 * 300.0 + 100.0).px(),
                                width: px(200.0),
                                height: px(200.0),
                            },
                            text_align: match x {
                                0 => TextAlign::Left,
                                1 => TextAlign::Center,
                                2 => TextAlign::Right,
                                _ => unreachable!(),
                            },
                            text_baseline: match y {
                                0 => TextBaseline::Top,
                                1 => TextBaseline::Middle,
                                2 => TextBaseline::Bottom,
                                _ => unreachable!(),
                            },
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
                            prevent_default_codes: &[],
                            focus: None,
                            on_edit_done: &|value| {
                                set_text_3x3.mutate(move |text_3x3| {
                                    text_3x3[x][y] = value;
                                });
                            },
                        },
                    );
                }
            }
        });
    }
}
