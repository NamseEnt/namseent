use crate::app::game::*;
use crate::component::*;
use namui::*;

impl Game {
    pub fn create_rendering_context(&self) -> RenderingContext {
        let namui_screen_size = namui::screen::size();
        let px_per_tile = Per::new(32.px(), 1.tile());
        let rendering_context = RenderingContext {
            px_per_tile,
            screen_rect: self.get_screen_rect(px_per_tile, namui_screen_size),
        };
        rendering_context
    }

    fn get_screen_rect(&self, px_per_tile: Per<Px, Tile>, namui_screen_size: Wh<Px>) -> Rect<Tile> {
        let camera_center_position = self.camera_center_xy();

        let screen_size = Wh {
            width: px_per_tile.invert() * namui_screen_size.width,
            height: px_per_tile.invert() * namui_screen_size.height,
        };

        let screen_center = (screen_size * 0.5).to_xy();
        Rect::from_xy_wh(camera_center_position - screen_center, screen_size)
    }

    fn camera_center_xy(&self) -> Xy<Tile> {
        match self.state.camera.subject {
            CameraSubject::Object { id } => {
                let Some(subject) = self.ecs_app.entities().find(|entity| entity.id() == id) else {
                    return Xy::zero();
                };
                let Some(positioner) = subject.get_component::<&Positioner>() else {
                    return Xy::zero();
                };
                positioner.xy
            }
            CameraSubject::Xy { xy } => xy.clone(),
        }
    }
}
