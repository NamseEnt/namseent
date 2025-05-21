use super::*;

pub(crate) fn render(game_state: &GameState, ctx: ComposeCtx<'_, '_>) {
    game_state.render_cursor_preview(&ctx);
    game_state.render_projectiles(&ctx);
    game_state.render_monsters(&ctx);
    game_state.render_route_guide(&ctx);
    game_state.render_towers(&ctx);
    game_state.render_grid(&ctx);
    game_state.render_backgrounds(&ctx);
}

// ASSUME: NO EFFECT AND STATE IN INNER RENDER
// Render in the 1:1 scale, without thinking about the camera zoom level.
impl GameState {
    fn render_grid(&self, ctx: &ComposeCtx) {
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
        ctx.add(namui::path(path, paint));
    }

    fn render_backgrounds(&self, ctx: &ComposeCtx) {
        self.render_stuffs(
            ctx,
            self.backgrounds
                .iter()
                .map(|background| (background.coord, background)),
        );
    }

    fn render_projectiles(&self, ctx: &ComposeCtx) {
        self.render_stuffs(
            ctx,
            self.projectiles
                .iter()
                .map(|projectile| (projectile.xy, projectile)),
        );
    }

    fn render_towers(&self, ctx: &ComposeCtx) {
        self.render_stuffs(ctx, self.towers.iter().map(|tower| (tower.left_top, tower)));
    }

    fn render_monsters(&self, ctx: &ComposeCtx) {
        self.render_stuffs(
            ctx,
            self.monsters
                .iter()
                .map(|monster| (monster.move_on_route.xy(), monster)),
        );
    }

    fn render_route_guide(&self, ctx: &ComposeCtx) {
        let mut path = Path::new();
        for coord in self.route.iter_coords() {
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

            let px_xy = TILE_PX_SIZE.as_xy() * xy.map(|t| t.as_f32());
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

    fn render_cursor_preview(&self, ctx: &ComposeCtx) {
        ctx.add(self.cursor_preview.render());
    }
}
