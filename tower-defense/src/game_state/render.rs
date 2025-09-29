use super::*;

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
    // Capture game_state reference to avoid unused variable warning
    let _towers = &game_state.towers;

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
        if let Event::MouseMove { event } = event {
            let local_xy = event.local_xy();
            let tile_x = (local_xy.x / TILE_PX_SIZE.width).floor() as usize;
            let tile_y = (local_xy.y / TILE_PX_SIZE.height).floor() as usize;

            mutate_game_state(move |game_state| {
                // Check if mouse is over any tower
                let tower_at_position = game_state.towers.find_by_xy(MapCoord::new(tile_x, tile_y));

                match tower_at_position {
                    Some(_) => {
                        // Mouse is over a tower, but the tower's own event handler will handle the hover
                    }
                    None => {
                        // Mouse is not over any tower, hide all hover states
                        game_state.set_hovered_tower(None);
                    }
                }
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
    let camera = &game_state.camera;
    let screen_rect = {
        let screen_size = namui::screen::size();
        Rect::from_xy_wh(camera.left_top, {
            Wh::new(
                screen_size.width.as_i32().as_f32() / TILE_PX_SIZE.width.as_f32(),
                screen_size.height.as_i32().as_f32() / TILE_PX_SIZE.height.as_f32(),
            ) / camera.zoom_level
        })
    };

    for tower in game_state.towers.iter() {
        let tower_xy = tower.left_top.map(|t| t.as_f32());

        // Culling check
        if screen_rect.right() < tower_xy.x || screen_rect.bottom() < tower_xy.y {
            continue;
        }

        let px_xy = TILE_PX_SIZE.to_xy() * tower_xy;
        ctx.translate(px_xy).compose(move |ctx| {
            // For now, just render the tower without hover functionality
            // We'll need to modify this once we can access mutable game state
            ctx.add(tower);

            // Render hover area
            let tower_size = 128.0; // TILE_PX_SIZE
            ctx.add(namui::rect(RectParam {
                rect: Rect::from_xy_wh(Xy::zero(), Wh::new(tower_size.px(), tower_size.px())),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::TRANSPARENT,
                    }),
                    ..Default::default()
                },
            }))
            .attach_event({
                println!("Attaching hover event for tower {:?}", tower.id());
                let tower_id = tower.id();
                move |event| {
                    let Event::MouseMove { event } = event else {
                        return;
                    };
                    if !event.is_local_xy_in() {
                        return;
                    }
                    mutate_game_state(move |game_state| {
                        game_state.set_hovered_tower(Some(tower_id));
                    });
                }
            });
        });
    }
}

fn render_tower_info_popup(ctx: &RenderCtx, game_state: &GameState) {
    use crate::game_state::tower_info_popup::TowerInfoPopup;

    for tower in game_state.towers.iter() {
        if let Some(hover_state) = game_state.ui_state.get_hover_state(tower.id())
            && hover_state.is_visible()
        {
            let tower_upgrades = game_state.upgrade_state.tower_upgrades(tower);

            let popup_scale = hover_state.scale;
            let popup_opacity = hover_state.opacity;

            if popup_scale > 0.01 && popup_opacity > 0.01 {
                let px_xy = TILE_PX_SIZE.to_xy() * tower.left_top.map(|t| t as f32)
                    + Xy::new(TILE_PX_SIZE.width, 0.px());

                ctx.translate(px_xy)
                    .scale(Xy::single(popup_scale / game_state.camera.zoom_level))
                    .add(TowerInfoPopup {
                        tower,
                        tower_upgrades: &tower_upgrades,
                        game_state,
                    });
            }
        }
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
