use super::*;
use crate::game_state::{can_place_tower::can_place_tower, tower_info_popup::TowerInfoPopup};

// ASSUME: NO EFFECT AND STATE IN INNER RENDER
// Render in the 1:1 scale, without thinking about the camera zoom level.
pub(crate) fn render(game_state: &GameState, ctx: ComposeCtx<'_, '_>) {
    ctx.add((render_tower_info_popup, game_state));
    ctx.add((render_cursor_preview, game_state));
    ctx.add((render_field_particles, game_state));
    ctx.add((render_projectiles, game_state));
    ctx.add((render_monsters, game_state));
    ctx.add((render_route_guide, game_state));
    ctx.add((render_towers, game_state));
    ctx.add((render_grid, game_state));
    ctx.add((render_backgrounds, game_state));
}

impl GameState {
    fn render_stuffs<'a, C, MapCoord, MapAxis>(
        &self,
        ctx: &ComposeCtx,
        stuffs: impl Iterator<Item = (MapCoord, C)>,
    ) where
        C: 'a + Component,
        MapCoord: AsRef<Xy<MapAxis>>,
        MapAxis: Ratio + std::fmt::Debug + Clone + Copy,
    {
        let camera = &self.camera;

        let screen_rect = Rect::from_xy_wh(camera.left_top, {
            let screen_size = namui::screen::size();
            Wh::new(
                screen_size.width.as_i32().as_f32() / TILE_PX_SIZE.width.as_f32(),
                screen_size.height.as_i32().as_f32() / TILE_PX_SIZE.height.as_f32(),
            ) / camera.zoom_level
        });

        for (xy, stuff) in stuffs {
            let xy = *xy.as_ref();
            if screen_rect.right() < xy.x.as_f32() || screen_rect.bottom() < xy.y.as_f32() {
                continue;
            }

            let px_xy = TILE_PX_SIZE.to_xy() * xy.map(|t| t.as_f32());
            ctx.translate(px_xy).compose(move |ctx| {
                let rendering_tree = ctx.ghost_add("", stuff);
                let Some(bounding_box) = namui::bounding_box(&rendering_tree) else {
                    return;
                };

                let local_right = bounding_box.right() / TILE_PX_SIZE.width;
                let local_bottom = bounding_box.bottom() / TILE_PX_SIZE.height;

                if xy.x.as_f32() + local_right < screen_rect.left()
                    || xy.y.as_f32() + local_bottom < screen_rect.top()
                {
                    return;
                }

                ctx.add(rendering_tree);
            });
        }
    }
}

fn render_grid(ctx: &RenderCtx, game_state: &GameState) {
    let mut path = Path::new();
    for x in 0..MAP_SIZE.width + 1 {
        let x = (x.as_f32() * TILE_PX_SIZE.width.as_f32()).px();
        path = path.move_to(x, 0.px());
        path = path.line_to(
            x,
            (MAP_SIZE.height.as_f32() * TILE_PX_SIZE.height.as_f32()).px(),
        );
    }
    for y in 0..MAP_SIZE.height + 1 {
        let y = (y.as_f32() * TILE_PX_SIZE.height.as_f32()).px();
        path = path.move_to(0.px(), y);
        path = path.line_to(
            (MAP_SIZE.width.as_f32() * TILE_PX_SIZE.width.as_f32()).px(),
            y,
        );
    }
    let paint = Paint::new(Color::grayscale_alpha_f01(1.0, 0.5))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.px())
        .set_stroke_cap(StrokeCap::Round);

    ctx.add(namui::path(path, paint)).attach_event(|event| {
        if let GameFlow::PlacingTower { .. } = game_state.flow
            && let Event::MouseUp { event } = event
            && let Some(MouseButton::Left) = event.button
        {
            let local_xy = event.local_xy();
            let tile_x = (local_xy.x / TILE_PX_SIZE.width).floor() as usize;
            let tile_y = (local_xy.y / TILE_PX_SIZE.height).floor() as usize;

            mutate_game_state(move |game_state| {
                let new_selected_tower_id = game_state
                    .towers
                    .find_by_xy(MapCoord::new(tile_x, tile_y))
                    .map(|tower| tower.id());
                game_state.selected_tower_id =
                    if game_state.selected_tower_id == new_selected_tower_id {
                        None
                    } else {
                        new_selected_tower_id
                    };
            });
        }
    });
}

fn render_backgrounds(ctx: &RenderCtx, game_state: &GameState) {
    game_state.render_stuffs(
        ctx,
        game_state
            .backgrounds
            .iter()
            .map(|background| (background.coord, background)),
    );
}

fn render_projectiles(ctx: &RenderCtx, game_state: &GameState) {
    game_state.render_stuffs(
        ctx,
        game_state
            .projectiles
            .iter()
            .map(|projectile| (projectile.xy, projectile)),
    );
}

fn render_towers(ctx: &RenderCtx, game_state: &GameState) {
    game_state.render_stuffs(
        ctx,
        game_state
            .towers
            .iter()
            .map(|tower| (tower.left_top, tower)),
    );
}

fn render_tower_info_popup(ctx: &RenderCtx, game_state: &GameState) {
    if let Some(selected_tower_id) = game_state.selected_tower_id
        && let Some(selected_tower) = game_state.towers.find_by_id(selected_tower_id)
    {
        let tower_upgrades = game_state.upgrade_state.tower_upgrades(selected_tower);

        let px_xy = TILE_PX_SIZE.to_xy() * selected_tower.left_top.map(|t| t as f32)
            + Xy::new(TILE_PX_SIZE.width, 0.px());
        ctx.translate(px_xy)
            .scale(Xy::single(1. / game_state.camera.zoom_level))
            .add(TowerInfoPopup {
                tower: selected_tower,
                tower_upgrades: &tower_upgrades,
            });
    }
}

fn render_monsters(ctx: &RenderCtx, game_state: &GameState) {
    game_state.render_stuffs(
        ctx,
        game_state
            .monsters
            .iter()
            .map(|monster| (monster.move_on_route.xy(), monster)),
    );
}

fn render_route_guide(ctx: &RenderCtx, game_state: &GameState) {
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

    let paint = Paint::new(Color::RED)
        .set_style(PaintStyle::Stroke)
        .set_stroke_cap(StrokeCap::Round);

    ctx.add(namui::path(path, paint));
}

fn render_cursor_preview(ctx: &RenderCtx, game_state: &GameState) {
    ctx.add(game_state.cursor_preview.render());

    // Render tower preview if in PlacingTower flow and hand has selected tower
    if let crate::game_state::GameFlow::PlacingTower { hand } = &game_state.flow {
        let selected_slot_ids = hand.selected_slot_ids();
        if let Some(&selected_slot_id) = selected_slot_ids.first()
            && let Some(tower_template) = hand.get_item(selected_slot_id)
        {
            ctx.add(
                crate::game_state::cursor_preview::tower::TowerCursorPreview {
                    tower_template,
                    map_coord: game_state.cursor_preview.map_coord,
                    placing_tower_slot_id: selected_slot_id,
                },
            );
        }
    }
}

fn render_field_particles(ctx: &RenderCtx, game_state: &GameState) {
    game_state
        .field_particle_system_manager
        .render(ctx, game_state.now());
}
