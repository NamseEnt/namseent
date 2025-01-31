use super::*;

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
        MapAxis: Ratio + std::fmt::Debug + Clone,
    {
        for (xy, stuff) in stuffs {
            let px_xy = xy
                .as_ref()
                .clone()
                .map(|t| self.camera.map_coord_to_screen_px_ratio() * t);
            ctx.translate(px_xy).add(stuff);
        }
    }
}
