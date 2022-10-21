use crate::app::game::*;
use namui::prelude::*;

impl Game {
    pub fn create_rendering_context(&self) -> RenderingContext {
        let screen_size = namui::screen::size();
        let current_time = now();
        let px_per_tile = Per::new(32.px(), 1.tile());
        let interpolation_progress = self.state.tick.interpolation_progress(current_time);
        let rendering_context = RenderingContext {
            current_time,
            px_per_tile,
            screen_rect: self.get_screen_rect(px_per_tile, screen_size, interpolation_progress),
            interpolation_progress,
        };
        rendering_context
    }

    fn get_screen_rect(
        &self,
        px_per_tile: Per<Px, Tile>,
        screen_size: Wh<Px>,
        interpolation_progress: f32,
    ) -> Rect<Tile> {
        let camera_center_position = self.camera.get_xy(&self.ecs_app, interpolation_progress);

        let screen_size = Wh {
            width: px_per_tile.invert() * screen_size.width,
            height: px_per_tile.invert() * screen_size.height,
        };

        let screen_center = (screen_size * 0.5).as_xy();
        Rect::from_xy_wh(camera_center_position - screen_center, screen_size)
    }
}
