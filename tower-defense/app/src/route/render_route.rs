use super::*;
use crate::game_state::TRAVEL_POINTS;
use namui::*;

pub fn render_route_guide(ctx: &RenderCtx) {
    let mut path = Path::new();
    for coord in TRAVEL_POINTS.iter() {
        let xy = Xy::new(
            (coord.x.as_f32() + 0.5) * TILE_PX_SIZE.width.as_f32(),
            (coord.y.as_f32() + 0.5) * TILE_PX_SIZE.height.as_f32(),
        )
        .map(px);
        if path.commands().is_empty() {
            path = path.move_to(xy.x, xy.y);
        } else {
            path = path.line_to(xy.x, xy.y);
        }
    }

    let path = path.stroke(StrokeOptions {
        width: Some(TILE_PX_SIZE.width * 0.65),
        miter_limit: None,
        precision: None,
        join: Some(StrokeJoin::Round),
        cap: Some(StrokeCap::Round),
    });

    let texture_paint = Paint::new(Color::WHITE)
        .set_style(PaintStyle::Fill)
        .set_shader(Shader::Image {
            src: asset::image::route::ROUTE_1,
            tile_mode: Xy::single(TileMode::Repeat),
        });

    let border_paint = Paint::new(Color::from_u8(205, 170, 125, 128))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(12.px());

    ctx.add(namui::path(path.clone(), texture_paint));
    ctx.add(namui::path(path, border_paint));
}
