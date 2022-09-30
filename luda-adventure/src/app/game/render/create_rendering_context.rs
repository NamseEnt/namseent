use crate::app::game::*;
use namui::prelude::*;

impl Game {
    pub fn create_rendering_context(&self) -> RenderingContext {
        let screen_size = namui::screen::size();
        let current_time = now();
        let px_per_tile = Per::new(32.px(), 1.tile());
        let rendering_context = RenderingContext {
            current_time,
            px_per_tile,
            screen_rect: self.get_screen_rect(current_time, px_per_tile, screen_size),
        };
        rendering_context
    }

    fn get_screen_rect(
        &self,
        time: Time,
        px_per_tile: Per<Px, Tile>,
        screen_size: Wh<Px>,
    ) -> Rect<Tile> {
        let camera_center_position = self.camera.get_position(&self.ecs_app, time);

        let screen_size = Wh {
            width: px_per_tile.invert() * screen_size.width,
            height: px_per_tile.invert() * screen_size.height,
        };

        let screen_center = (screen_size * 0.5).as_xy();
        Rect::from_xy_wh(camera_center_position - screen_center, screen_size)
    }
}
