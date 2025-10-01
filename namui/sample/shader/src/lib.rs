use namui::*;
use namui_prebuilt::{table::*, *};
use std::f32::consts::PI;

pub fn main() {
    namui::start(&mut ShaderExample::new(), &()).await
}

#[derive(Debug, bincode::Decode, bincode::Encode, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Spiral,
    Shake,
    Scroll,
}
impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tab::Spiral => write!(f, "Spiral"),
            Tab::Shake => write!(f, "Shake"),
            Tab::Scroll => write!(f, "Scroll"),
        }
    }
}

struct ShaderExample {
    tab: Tab,
}

impl ShaderExample {
    fn new() -> Self {
        Self { tab: Tab::Scroll }
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
                    items: vec![Tab::Spiral, Tab::Shake, Tab::Scroll]
                        .into_iter()
                        .map(|tab| dropdown::Item {
                            is_selected: self.tab == tab,
                            text: tab.to_string(),
                            on_select_item: move |_| namui::event::send(Event::SelectTab { tab }),
                        }),
                    rect: Rect::from_xy_wh(
                        Xy::single(0.px()),
                        Wh {
                            width: wh.width,
                            height: wh.height.min(40.px()),
                        },
                    ),
                    visible_item_count: 0,
                })
            }),
            ratio(1.0, |wh| match self.tab {
                Tab::Spiral => {
                    namui::shader!(SpiralShader, {
                        uniform float rad_scale;
                        uniform float2 in_center;
                        uniform float4 in_colors0;
                        uniform float4 in_colors1;

                        half4 main(float2 xy) {
                            float2 pp = xy - in_center;
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
                    let paint = Paint::new().set_shader(shader.make());
                    let rect = Path::new().add_rect(Rect::Xywh {
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
                        uniform float2 wh;
                        uniform float delta_x_center;
                        uniform float2 source_image_size;
                        uniform float2 dest_image_size;
                        uniform float2 image_left_top;
                        uniform shader image;


                        half4 main(float2 xy) {
                            float pi = 3.1415926;
                            float2 image_local_xy = (xy - image_left_top) * source_image_size / dest_image_size;


                            float2 result_xy = image_local_xy + float2(
                                sin(image_local_xy.y / (source_image_size.y / 2.0) * pi / 2) * delta_x_center,
                                0.0
                            );

                            return image.eval(result_xy).rgba;
                        }
                    });
                    let image_source_url = Url::parse("resources/test.jpg").unwrap();

                    let image = namui::image::try_load_url(&image_source_url);
                    if image.is_none() {
                        return RenderingTree::Empty;
                    }
                    let image = image.unwrap();
                    let source_image_size = image.size().into_slice();
                    let dest_image_size = (image.size() / 2.0).into_slice();
                    let start_x = 50.0;
                    let end_x = 800.0;
                    let rest_secs = 0.6;
                    let working_secs = 0.4;
                    let total_secs = working_secs + rest_secs;
                    let seconds = namui::now().as_seconds() % total_secs;
                    let time_ratio = seconds / working_secs;

                    let image_left_top = Xy {
                        x: {
                            if seconds > working_secs {
                                end_x
                            } else {
                                fn ease_curve(x: f32, alpha: f32) -> f32 {
                                    x.powf(alpha) / (x.powf(alpha) + (1.0 - x).powf(alpha))
                                }
                                let curved_time_ratio = ease_curve(time_ratio, 6.0);

                                start_x + (end_x - start_x) * curved_time_ratio
                            }
                        },
                        y: 50.0,
                    }
                    .as_slice();

                    let display_rect =
                        Rect::from_xy_wh(Xy::single(100.px()), wh - Wh::single(200.px()));

                    let wh = display_rect.wh().into_slice();
                    let delta_x_center = {
                        if seconds < working_secs {
                            (time_ratio / 0.5 * PI).sin() * image.size().width.as_f32() / 2.0
                        } else {
                            ((time_ratio - 1.0) / 0.5 * 2.0 * PI).sin()
                                * image.size().width.as_f32()
                                / (2.0 * time_ratio.powi(4))
                        }
                    };

                    let image_shader = image.make_shader(
                        TileMode::Decal,
                        TileMode::Decal,
                        FilterMode::Linear,
                        MipmapMode::Linear,
                    );

                    let shader = ShakeShader::new(
                        wh,
                        delta_x_center,
                        source_image_size,
                        dest_image_size,
                        image_left_top,
                        image_shader,
                    );
                    let paint = Paint::new().set_shader(shader.make());
                    let rect = Path::new()
                        .add_rect(Rect::from_xy_wh(Xy::zero().into(), display_rect.wh()));

                    translate(
                        display_rect.x(),
                        display_rect.y(),
                        render([
                            path(rect, paint),
                            simple_rect(display_rect.wh(), Color::RED, 1.px(), Color::TRANSPARENT),
                        ]),
                    )
                }
                Tab::Scroll => {
                    namui::shader!(ScrollShader, {
                        uniform float2 xy;
                        uniform float2 wh;
                        uniform float delta_y;
                        uniform float2 image_wh;
                        uniform shader image;

                        float2 local_xy(float2 xy) {
                            return float2(
                                (xy.x - xy.x) / (wh.x / image_wh.x),
                                (xy.y - xy.y) / (wh.y / image_wh.y)
                            );
                        }
                        half4 main(float2 xy) {
                            return image.eval(local_xy(xy - float2(0.0, delta_y))).rgba;
                        }
                    });

                    let image_source_url = Url::parse("resources/sweat.png").unwrap();

                    let image = namui::image::try_load_url(&image_source_url);
                    if image.is_none() {
                        return RenderingTree::Empty;
                    }
                    let image = image.unwrap();

                    let image_shader = image.make_shader(
                        namui::TileMode::Repeat,
                        namui::TileMode::Repeat,
                        namui::FilterMode::Linear,
                        namui::MipmapMode::Linear,
                    );

                    let xy = [100.0, 100.0];
                    let wh = [200.0, 200.0];
                    let image_wh = image.size();
                    let image_wh = [image_wh.width.as_f32(), image_wh.height.as_f32()];
                    let delta_y = -(namui::now().as_seconds() * wh[1] * 3.0);

                    let shader = ScrollShader::new(xy, wh, delta_y, image_wh, image_shader);
                    let paint = Paint::new().set_shader(shader.make());
                    let rect = Path::new().add_rect(Rect::Xywh {
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

    fn update(&mut self, event: &namui::Event) {
        if let Some(Event::SelectTab { tab }) = event.downcast_ref() {
            self.tab = *tab;
        }
    }
}
