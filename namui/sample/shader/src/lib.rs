use std::f32::consts::PI;

use namui::types::State;
use namui::*;
use namui_prebuilt::{button::TextButton, simple_rect, table};

register_assets!();

pub fn main() {
    namui::start(|ctx| {
        ctx.add(ShaderExample);
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
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

struct ShaderExample;

impl Component for ShaderExample {
    fn render(self, ctx: &RenderCtx) {
        let (selected_tab, set_selected_tab) = ctx.state(|| Tab::Scroll);

        ctx.compose(|ctx| {
            // Button menu - always show all 3 tabs
            table::vertical([
                table::fixed(
                    48.px(),
                    table::horizontal([Tab::Spiral, Tab::Shake, Tab::Scroll].into_iter().map(
                        |tab| {
                            table::ratio(1, move |wh, ctx| {
                                ctx.add(TextButton {
                                    rect: wh.to_rect(),
                                    text: tab.to_string(),
                                    text_color: Color::WHITE,
                                    stroke_color: Color::WHITE,
                                    stroke_width: 1.px(),
                                    fill_color: if *selected_tab == tab {
                                        Color::BLUE
                                    } else {
                                        Color::grayscale_u8(64)
                                    },
                                    mouse_buttons: vec![MouseButton::Left],
                                    on_mouse_up_in: move |_| {
                                        set_selected_tab.set(tab);
                                    },
                                });
                            })
                        },
                    )),
                ),
                table::ratio(1, |wh, ctx| match *selected_tab {
                    Tab::Spiral => render_spiral_shader(wh, &ctx),
                    Tab::Shake => render_shake_shader(wh, &ctx),
                    Tab::Scroll => render_scroll_shader(wh, &ctx),
                }),
            ])(screen::size().into_type(), ctx);
        });
    }
}

fn render_spiral_shader(wh: Wh<Px>, ctx: &ComposeCtx) {
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

    let secs = namui::system::time::since_start().as_secs_f32();
    let red_scale = (secs / 2.0).sin() / 5.0;
    let in_center = [(wh.width / 2.0).as_f32(), (wh.height / 2.0).as_f32()];
    let in_colors0 = [1.0, 0.0, 0.0, 1.0];
    let in_colors1 = [0.0, 1.0, 0.0, 1.0];

    let shader = SpiralShader::new(red_scale, in_center, in_colors0, in_colors1);
    let paint = Paint::new(Color::BLACK).set_shader(shader.make());

    let rect = wh.to_rect();
    let path = Path::new().add_rect(rect);

    ctx.add(PathDrawCommand { path, paint });
    ctx.add(simple_rect(wh, Color::RED, 1.px(), Color::TRANSPARENT));
}

fn render_shake_shader(wh: Wh<Px>, ctx: &ComposeCtx) {
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

    let image = crate::asset::TEST;
    let source_image_size = image.info().wh().into_slice();
    let dest_image_size = (image.info().wh() / 2.0).into_slice();
    let start_x = 0.0;
    let end_x = wh.width.as_f32();
    let rest_secs = 0.6;
    let working_secs = 0.4;
    let total_secs = working_secs + rest_secs;
    let seconds = namui::time::since_start().as_secs_f32() % total_secs;
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
        y: 0.0,
    }
    .as_slice();

    let shader_wh = [wh.width.as_f32(), wh.height.as_f32()];
    let delta_x_center = {
        if seconds < working_secs {
            (time_ratio / 0.5 * PI).sin() * image.info().wh().width.as_f32() / 2.0
        } else {
            ((time_ratio - 1.0) / 0.5 * 2.0 * PI).sin() * image.info().wh().width.as_f32()
                / (2.0 * time_ratio.powi(4))
        }
    };

    let image_shader = Shader::Image {
        src: image,
        tile_mode: Xy::single(TileMode::Repeat),
    };
    let shader = ShakeShader::new(
        shader_wh,
        delta_x_center,
        source_image_size,
        dest_image_size,
        image_left_top,
        image_shader.into(),
    );
    let paint = Paint::new(Color::BLACK).set_shader(shader.make());
    let path = Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), wh));

    ctx.add(PathDrawCommand { path, paint });
    ctx.add(simple_rect(wh, Color::RED, 1.px(), Color::TRANSPARENT));
}

fn render_scroll_shader(wh: Wh<Px>, ctx: &ComposeCtx) {
    namui::shader!(ScrollShader, {
        uniform float delta_y;
        uniform shader image;

        half4 main(float2 xy) {
            return image.eval(xy - float2(0.0, delta_y)).rgba;
        }
    });

    let image = crate::asset::SWEAT;

    let image_shader = Shader::Image {
        src: image,
        tile_mode: Xy::single(TileMode::Repeat),
    };

    let secs = namui::system::time::since_start().as_secs_f32();
    let shader_wh = [wh.width.as_f32(), wh.height.as_f32()];
    let delta_y = -(secs * shader_wh[1]);

    let shader = ScrollShader::new(delta_y, image_shader.into());
    let paint = Paint::new(Color::BLACK).set_shader(shader.make());

    let rect = wh.to_rect();
    let path = Path::new().add_rect(rect);

    ctx.add(PathDrawCommand { path, paint });
    ctx.add(simple_rect(wh, Color::RED, 1.px(), Color::TRANSPARENT));
}
