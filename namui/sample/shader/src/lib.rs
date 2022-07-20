use std::{f32::consts::PI, mem::discriminant};

use namui::prelude::*;
use namui_prebuilt::{table::*, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn start() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut ShaderExample::new(), &()).await
}

#[derive(Debug, Clone, Copy)]
enum Tab {
    Spiral,
    Shake,
}

struct ShaderExample {
    tab: Tab,
}

impl ShaderExample {
    fn new() -> Self {
        Self { tab: Tab::Shake }
    }
}

enum Event {
    SelectTab { tab: Tab },
}

impl Entity for ShaderExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let size = namui::screen::size();

        horizontal([
            fixed(100.px(), |wh| {
                dropdown::render(dropdown::Props {
                    items: vec![
                        dropdown::Item {
                            id: "Spiral".to_string(),
                            is_selected: discriminant(&self.tab) == discriminant(&Tab::Spiral),
                            text: "Spiral".to_string(),
                        },
                        dropdown::Item {
                            id: "Shake".to_string(),
                            is_selected: discriminant(&self.tab) == discriminant(&Tab::Shake),
                            text: "Shake".to_string(),
                        },
                    ],
                    rect: Rect::from_xy_wh(
                        Xy::single(0.px()),
                        Wh {
                            width: wh.width,
                            height: wh.height.min(40.px()),
                        },
                    ),
                    on_select_item: |id| match id.as_str() {
                        "Spiral" => {
                            namui::event::send(Event::SelectTab { tab: Tab::Spiral });
                        }
                        "Shake" => {
                            namui::event::send(Event::SelectTab { tab: Tab::Shake });
                        }
                        _ => unreachable!(),
                    },
                    visible_item_count: 0,
                })
            }),
            ratio(1.0, |_wh| match self.tab {
                Tab::Spiral => {
                    namui::shader!(SpiralShader, {
                        uniform float rad_scale;
                        uniform float2 in_center;
                        uniform float4 in_colors0;
                        uniform float4 in_colors1;

                        half4 main(float2 p) {
                            float2 pp = p - in_center;
                            float radius = sqrt(dot(pp, pp));
                            radius = sqrt(radius);
                            float angle = atan(pp.y / pp.x);
                            float t = (angle + 3.1415926/2) / (3.1415926);
                            t += radius * rad_scale;
                            t = fract(t);
                            return half4(mix(in_colors0, in_colors1, t));
                        }
                    });

                    let red_scale = (namui::now().as_millis() / 2000.0).sin() / 5.0;
                    let in_center = [200.0, 200.0];
                    let in_colors0 = [1.0, 0.0, 0.0, 1.0];
                    let in_colors1 = [0.0, 1.0, 0.0, 1.0];

                    let shader = SpiralShader::new(red_scale, in_center, in_colors0, in_colors1);
                    let paint = PaintBuilder::new().set_shader(shader);
                    let rect = PathBuilder::new().add_rect(Rect::Xywh {
                        x: 100.px(),
                        y: 100.px(),
                        width: 200.px(),
                        height: 200.px(),
                    });

                    render([
                        path(rect, paint),
                        translate(
                            100.px(),
                            100.px(),
                            simple_rect(
                                Wh {
                                    width: 200.px(),
                                    height: 200.px(),
                                },
                                Color::RED,
                                1.px(),
                                Color::TRANSPARENT,
                            ),
                        ),
                    ])
                }
                Tab::Shake => {
                    namui::shader!(ShakeShader, {
                        uniform float2 xy;
                        uniform float2 wh;
                        uniform float delta_x_center;

                        half4 value(float2 p) {
                            return half4((p.x - xy.x) / wh.x, (p.y - xy.y) / wh.y, 0.0, 1.0);
                        }
                        half4 main(float2 p) {
                            float y_center = xy.y + wh.y / 2.0;
                            float value_x = p.x + sin((p.y - xy.y) / (y_center - xy.y) * 3.1415926 / 2) * delta_x_center;
                            if (value_x < xy.x) {
                                return half4(1.0, 0.0, 0.0, 1.0);
                            } else if (value_x > xy.x + wh.x) {
                                return half4(0.0, 1.0, 0.0, 1.0);
                            }
                            return half4(value_x / wh.x, value_x / wh.x, value_x / wh.x, 1.0);
                        }
                    });

                    let xy = [100.0, 100.0];
                    let wh = [200.0, 200.0];
                    let delta_x_center = (namui::now().as_seconds() * 2.0 * PI).sin() * 100.0;

                    let shader = ShakeShader::new(xy, wh, delta_x_center);
                    let paint = PaintBuilder::new().set_shader(shader);
                    let rect = PathBuilder::new().add_rect(Rect::Xywh {
                        x: 100.px(),
                        y: 100.px(),
                        width: 200.px(),
                        height: 200.px(),
                    });

                    render([
                        path(rect, paint),
                        translate(
                            100.px(),
                            100.px(),
                            simple_rect(
                                Wh {
                                    width: 200.px(),
                                    height: 200.px(),
                                },
                                Color::RED,
                                1.px(),
                                Color::TRANSPARENT,
                            ),
                        ),
                    ])
                }
            }),
        ])(size)
    }

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(Event::SelectTab { tab }) = event.downcast_ref::<Event>() {
            self.tab = *tab;
        }
    }
}
