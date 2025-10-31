mod shake;
pub use shake::ShakeIntensity;

use crate::{
    MapCoordF32,
    game_state::{MAP_SIZE, TILE_PX_SIZE},
};
use namui::*;

#[derive(State)]
pub struct Camera {
    pub(self) left_top: MapCoordF32,
    pub zoom_level: f32,
    pub shake_intensity: f32,
    pub visual_left_top: MapCoordF32,
}
impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            left_top: MapCoordF32::new(0.0, 0.0),
            zoom_level: 1.0,
            shake_intensity: 0.0,
            visual_left_top: MapCoordF32::new(0.0, 0.0),
        }
    }

    pub fn zoom(&mut self, delta: f32, origin_screen_xy: Xy<Px>) {
        let screen_wh = screen::size().into_type::<Px>();
        let prev_zoom_level = self.zoom_level;
        let next_zoom_level = (self.zoom_level + delta).clamp(max_zoom_out_level(), 1.0);
        let tile_delta = ((screen_wh / TILE_PX_SIZE) * (prev_zoom_level - next_zoom_level)
            / (prev_zoom_level * next_zoom_level))
            .to_xy();
        let ratio = origin_screen_xy / screen_wh.to_xy();
        self.left_top -= tile_delta * ratio;
        self.zoom_level = next_zoom_level;
        self.constrain_to_map();
    }

    pub fn move_by(&mut self, screen_px_xy: Xy<Px>) {
        let px_xy_on_1_0 = screen_px_xy / self.zoom_level;
        self.left_top += Xy::new(
            px_xy_on_1_0.x / TILE_PX_SIZE.width,
            px_xy_on_1_0.y / TILE_PX_SIZE.height,
        );
        self.constrain_to_map();
    }

    fn constrain_to_map(&mut self) {
        let screen_wh = screen::size().into_type::<Px>() / self.zoom_level;
        let half_screen_tiles = Xy::new(
            screen_wh.width.as_f32() / (2.0 * TILE_PX_SIZE.width.as_f32()),
            screen_wh.height.as_f32() / (2.0 * TILE_PX_SIZE.height.as_f32()),
        );

        self.left_top.x = self.left_top.x.clamp(
            -half_screen_tiles.x,
            MAP_SIZE.width as f32 - half_screen_tiles.x,
        );
        self.left_top.y = self.left_top.y.clamp(
            -half_screen_tiles.y,
            MAP_SIZE.height as f32 - half_screen_tiles.y,
        );

        // base 위치가 변했으니 visual도 즉시 갱신
        self.visual_left_top = self.left_top;
    }
}

fn max_zoom_out_level() -> f32 {
    let screen_wh = screen::size().map(|x| x.as_i32() as f32);
    let shorter_side = screen_wh.width.min(screen_wh.height) as f32;
    shorter_side / (MAP_SIZE.width as f32 * TILE_PX_SIZE.width.as_f32())
}
