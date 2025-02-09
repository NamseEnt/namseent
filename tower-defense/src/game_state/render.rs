use super::*;

pub(crate) fn render(game_state: &GameState, ctx: ComposeCtx<'_, '_>) {
    game_state.render_monsters(&ctx);
    game_state.render_towers(&ctx);
    game_state.render_floor_tiles(&ctx);
}

// ASSUME: NO EFFECT AND STATE IN INNER RENDER
// Render in the 1:1 scale, without thinking about the camera zoom level.
impl GameState {
    pub fn render_floor_tiles(&self, ctx: &ComposeCtx) {
        self.render_stuffs(ctx, self.floor_tiles.iter());
    }

    pub fn render_towers(&self, ctx: &ComposeCtx) {
        self.render_stuffs(ctx, self.towers.iter());
    }

    pub fn render_monsters(&self, ctx: &ComposeCtx) {
        self.render_stuffs(
            ctx,
            self.monsters
                .iter()
                .map(|monster| (monster.move_on_route.xy(), monster)),
        );
    }

    fn render_stuffs<'a, C, MapCoord, MapAxis>(
        &self,
        ctx: &ComposeCtx,
        stuffs: impl Iterator<Item = (MapCoord, &'a C)>,
    ) where
        C: 'a,
        &'a C: Component,
        MapCoord: AsRef<Xy<MapAxis>>,
        MapAxis: Ratio + std::fmt::Debug + Clone + Copy,
    {
        let camera = &self.camera;

        let screen_rect = Rect::from_xy_wh(
            camera.left_top,
            namui::screen::size().map(|t| t.into_px() / camera.map_coord_to_screen_px_ratio()),
        );

        for (xy, stuff) in stuffs {
            let xy = *xy.as_ref();
            if screen_rect.right() < xy.x.as_f32() || screen_rect.bottom() < xy.y.as_f32() {
                continue;
            }

            let px_xy = xy.map(|t| self.camera.map_coord_to_screen_px_ratio() * t);
            ctx.translate(px_xy).compose(move |ctx| {
                let rendering_tree = ctx.ghost_add("", stuff);
                let Some(bounding_box) = namui::bounding_box(&rendering_tree) else {
                    return;
                };

                let local_right = bounding_box.right() / self.camera.map_coord_to_screen_px_ratio();
                let local_bottom =
                    bounding_box.bottom() / self.camera.map_coord_to_screen_px_ratio();

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
