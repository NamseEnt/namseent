use super::*;
use crate::game_state::{
    can_place_tower::can_place_tower,
    flow::GameFlow,
    tower::{Tower, TowerTemplate},
    *,
};
use namui::*;

pub fn render_route_guide(ctx: &RenderCtx, game_state: &GameState) {
    // Route texture is fixed to 1.jpg (previously cycled with F8)
    let cursor_coord = ctx.track_eq(
        &game_state
            .cursor_preview
            .map_coord
            .map(|f| f.round() as usize),
    );
    let game_state_route = ctx.track_eq(&game_state.route);
    let towers = ctx.track_eq(&game_state.towers);
    let is_tower_placing =
        ctx.track_eq(&if let GameFlow::PlacingTower { hand } = &game_state.flow {
            !hand.selected_slot_ids().is_empty()
        } else {
            false
        });

    let route = ctx.memo(|| {
        'placing_tower: {
            if cursor_coord.x == 0 || cursor_coord.y == 0 || !*is_tower_placing {
                break 'placing_tower;
            }
            let cursor_tower_coord = cursor_coord.as_ref().map(|v| v - 1);
            if !can_place_tower(
                cursor_tower_coord,
                Wh::single(2),
                &TRAVEL_POINTS,
                &towers.coords(),
                game_state_route.iter_coords(),
                MAP_SIZE,
            ) {
                break 'placing_tower;
            }
            let mut towers = towers.clone_inner();
            towers.place_tower(Tower::new(
                &TowerTemplate::barricade(),
                cursor_tower_coord,
                game_state.now(),
            ));
            return calculate_routes(&towers.coords(), &TRAVEL_POINTS, MAP_SIZE).unwrap();
        };
        game_state_route.clone_inner()
    });

    // Removed F8 key handler that changed route texture.

    let mut path = Path::new();
    for coord in route.iter_coords() {
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
