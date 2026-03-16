use namui::*;

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        ctx.add(ClipTest);
    })
}

struct ClipTest;

const RECT_W: Px = px(120.0);
const RECT_H: Px = px(120.0);
const GAP: Px = px(20.0);
const CLIP_HEIGHT: Px = px(60.0);
const LABEL_HEIGHT: Px = px(20.0);
const START_X: Px = px(40.0);
const START_Y: Px = px(60.0);
const ROW2_Y: Px = px(260.0);

impl Component for ClipTest {
    fn render(self, ctx: &RenderCtx) {
        let row1: [(&str, fn(&RenderCtx)); 5] = [
            ("1. Solid Fill", render_solid_fill),
            ("2. Gradient Shader", render_gradient_shader),
            ("3. MaskFilter Blur", render_mask_filter),
            ("4. Stroke 4px", render_stroke),
            ("5. ColorFilter", render_color_filter),
        ];

        let row2: [(&str, fn(&RenderCtx)); 5] = [
            ("6. Star Fill", render_star_fill),
            ("7. Stroke+MaskFilter", render_stroke_mask_filter),
            ("8. Fill Circle", render_fill_circle),
            ("9. Stroke Circle", render_stroke_circle),
            ("10. Fill+Stroke", render_fill_and_stroke),
        ];

        render_row(ctx, &row1, START_Y);
        render_row(ctx, &row2, ROW2_Y);

        ctx.translate((START_X, ROW2_Y + RECT_H + GAP * 2.0))
            .add(namui::text(TextParam {
                text: "Red dashed line = clip boundary (60px). Content below it should be clipped."
                    .to_string(),
                x: 0.px(),
                y: 0.px(),
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                font: Font {
                    size: 14.int_px(),
                    name: "NotoSansKR-Regular".to_string(),
                },
                style: TextStyle {
                    color: Color::WHITE,
                    ..Default::default()
                },
                max_width: None,
            }));
    }
}

fn render_row(ctx: &RenderCtx, items: &[(&str, fn(&RenderCtx))], y: Px) {
    for (i, (label, render_fn)) in items.iter().enumerate() {
        let x = START_X + (RECT_W + GAP) * i;

        ctx.translate((x, y - LABEL_HEIGHT))
            .add(namui::text(TextParam {
                text: label.to_string(),
                x: 0.px(),
                y: 0.px(),
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                font: Font {
                    size: 12.int_px(),
                    name: "NotoSansKR-Regular".to_string(),
                },
                style: TextStyle {
                    color: Color::WHITE,
                    ..Default::default()
                },
                max_width: None,
            }));

        ctx.translate((x, y))
            .clip(
                Path::new().add_rect(Rect::Xywh {
                    x: 0.px(),
                    y: 0.px(),
                    width: RECT_W,
                    height: CLIP_HEIGHT,
                }),
                ClipOp::Intersect,
            )
            .compose(|ctx| {
                ctx.add(RenderItem { render_fn: *render_fn });
            });

        ctx.translate((x, y)).add(clip_boundary_indicator());
    }
}

struct RenderItem {
    render_fn: fn(&RenderCtx),
}

impl Component for RenderItem {
    fn render(self, ctx: &RenderCtx) {
        (self.render_fn)(ctx);
    }
}

fn clip_boundary_indicator() -> RenderingTree {
    let paint = Paint::new(Color::RED)
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(1.px())
        .set_path_effect(PathEffect::Dash {
            on: 4.0,
            off: 4.0,
            phase: 0.0,
        });
    namui::path(
        Path::new().add_rect(Rect::Xywh {
            x: 0.px(),
            y: 0.px(),
            width: RECT_W,
            height: CLIP_HEIGHT,
        }),
        paint,
    )
}

fn render_solid_fill(ctx: &RenderCtx) {
    let paint = Paint::new(Color::from_u8(0, 120, 255, 255)).set_style(PaintStyle::Fill);
    ctx.add(namui::path(
        Path::new().add_rect(Rect::Xywh {
            x: 0.px(),
            y: 0.px(),
            width: RECT_W,
            height: RECT_H,
        }),
        paint,
    ));
}

fn render_gradient_shader(ctx: &RenderCtx) {
    let paint = Paint::new(Color::WHITE)
        .set_style(PaintStyle::Fill)
        .set_shader(Shader::LinearGradient {
            start_xy: Xy::new(0.px(), 0.px()),
            end_xy: Xy::new(RECT_W, RECT_H),
            colors: vec![
                Color::from_u8(255, 0, 0, 255),
                Color::from_u8(0, 0, 255, 255),
            ],
            tile_mode: TileMode::Clamp,
        });
    ctx.add(namui::path(
        Path::new().add_rect(Rect::Xywh {
            x: 0.px(),
            y: 0.px(),
            width: RECT_W,
            height: RECT_H,
        }),
        paint,
    ));
}

fn render_mask_filter(ctx: &RenderCtx) {
    let paint = Paint::new(Color::from_u8(0, 0, 0, 192))
        .set_style(PaintStyle::Fill)
        .set_mask_filter(MaskFilter::Blur {
            blur_style: BlurStyle::Normal,
            sigma: 4.0,
        });
    ctx.add(namui::path(
        Path::new().add_rect(Rect::Xywh {
            x: 10.px(),
            y: 10.px(),
            width: RECT_W - 20.px(),
            height: RECT_H - 20.px(),
        }),
        paint,
    ));
}

fn render_stroke(ctx: &RenderCtx) {
    let paint = Paint::new(Color::from_u8(0, 200, 0, 255))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(4.px());
    ctx.add(namui::path(
        Path::new().add_rect(Rect::Xywh {
            x: 10.px(),
            y: 10.px(),
            width: RECT_W - 20.px(),
            height: RECT_H - 20.px(),
        }),
        paint,
    ));
}

fn render_color_filter(ctx: &RenderCtx) {
    let paint = Paint::new(Color::from_u8(255, 165, 0, 255))
        .set_style(PaintStyle::Fill)
        .set_color_filter(ColorFilter::Blend {
            color: Color::from_u8(0, 0, 255, 128),
            blend_mode: BlendMode::SrcOver,
        });
    ctx.add(namui::path(
        Path::new().add_rect(Rect::Xywh {
            x: 0.px(),
            y: 0.px(),
            width: RECT_W,
            height: RECT_H,
        }),
        paint,
    ));
}

fn render_star_fill(ctx: &RenderCtx) {
    let paint = Paint::new(Color::from_u8(255, 215, 0, 255)).set_style(PaintStyle::Fill);
    let cx = RECT_W / 2.0;
    let cy = RECT_H / 2.0;
    let outer_r = 55.0_f32;
    let inner_r = 22.0_f32;
    let mut path = Path::new();
    for i in 0..10 {
        let angle = std::f32::consts::PI / 5.0 * i as f32 - std::f32::consts::FRAC_PI_2;
        let r = if i % 2 == 0 { outer_r } else { inner_r };
        let x = cx + px(r * angle.cos());
        let y = cy + px(r * angle.sin());
        if i == 0 {
            path = path.move_to(x, y);
        } else {
            path = path.line_to(x, y);
        }
    }
    path = path.close();
    ctx.add(namui::path(path, paint));
}

fn render_stroke_mask_filter(ctx: &RenderCtx) {
    let paint = Paint::new(Color::from_u8(0, 200, 0, 255))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(4.px())
        .set_mask_filter(MaskFilter::Blur {
            blur_style: BlurStyle::Normal,
            sigma: 2.0,
        });
    ctx.add(namui::path(
        Path::new().add_rect(Rect::Xywh {
            x: 10.px(),
            y: 10.px(),
            width: RECT_W - 20.px(),
            height: RECT_H - 20.px(),
        }),
        paint,
    ));
}

fn render_fill_circle(ctx: &RenderCtx) {
    let paint = Paint::new(Color::from_u8(200, 0, 200, 255)).set_style(PaintStyle::Fill);
    ctx.add(namui::path(
        Path::new().add_oval(Rect::Xywh {
            x: 10.px(),
            y: 10.px(),
            width: RECT_W - 20.px(),
            height: RECT_H - 20.px(),
        }),
        paint,
    ));
}

fn render_stroke_circle(ctx: &RenderCtx) {
    let paint = Paint::new(Color::from_u8(0, 200, 200, 255))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(4.px());
    ctx.add(namui::path(
        Path::new().add_oval(Rect::Xywh {
            x: 10.px(),
            y: 10.px(),
            width: RECT_W - 20.px(),
            height: RECT_H - 20.px(),
        }),
        paint,
    ));
}

fn render_fill_and_stroke(ctx: &RenderCtx) {
    let fill_paint = Paint::new(Color::from_u8(0, 120, 255, 128)).set_style(PaintStyle::Fill);
    let stroke_paint = Paint::new(Color::from_u8(255, 255, 255, 255))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(3.px());
    let rect = Rect::Xywh {
        x: 10.px(),
        y: 10.px(),
        width: RECT_W - 20.px(),
        height: RECT_H - 20.px(),
    };
    ctx.add(namui::path(Path::new().add_rect(rect), fill_paint));
    ctx.add(namui::path(Path::new().add_rect(rect), stroke_paint));
}
