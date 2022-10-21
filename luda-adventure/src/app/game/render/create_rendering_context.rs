use crate::app::game::*;
use namui::prelude::*;

impl Game {
    pub fn create_rendering_context(&self) -> RenderingContext {
        let namui_screen_size = namui::screen::size();
        let current_time = now();
        let px_per_tile = Per::new(32.px(), 1.tile());
        let interpolation_progress = self.state.tick.interpolation_progress(current_time);
        let rendering_context = RenderingContext {
            current_time,
            px_per_tile,
            screen_rect: self.get_screen_rect(
                px_per_tile,
                namui_screen_size,
                interpolation_progress,
            ),
            interpolation_progress,
        };
        rendering_context
    }

    fn get_screen_rect(
        &self,
        px_per_tile: Per<Px, Tile>,
        namui_screen_size: Wh<Px>,
        interpolation_progress: f32,
    ) -> Rect<Tile> {
        let camera_center_position = self.camera_center_xy(interpolation_progress);

        let screen_size = Wh {
            width: px_per_tile.invert() * namui_screen_size.width,
            height: px_per_tile.invert() * namui_screen_size.height,
        };

        let screen_center = (screen_size * 0.5).as_xy();
        Rect::from_xy_wh(camera_center_position - screen_center, screen_size)
    }

    fn camera_center_xy(&self, interpolation_progress: f32) -> Xy<Tile> {
        match self.state.camera.subject() {
            CameraSubject::Object { id } => self
                .ecs_app
                .entities()
                .find(|entity| entity.id() == id)
                .expect("failed to find entity")
                .get_component::<&Positioner>()
                .unwrap()
                .xy_with_interpolation(interpolation_progress),
            CameraSubject::Xy { xy } => xy.clone(),
        }
    }
}
