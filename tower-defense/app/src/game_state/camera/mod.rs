mod shake;
pub use shake::ShakeIntensity;

use crate::{
    MapCoordF32,
    game_state::{MAP_OUTSIDE_MARGIN_TILES, MAP_SIZE, TILE_PX_SIZE},
};
use namui::*;

#[derive(State, Clone)]
pub struct Camera {
    pub(self) left_top: MapCoordF32,
    pub zoom_level: f32,
    pub shake_intensity: f32,
    shake_offset: Xy<f32>,
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
            shake_offset: Xy::new(0.0, 0.0),
        }
    }

    pub fn visual_left_top(&self) -> MapCoordF32 {
        self.left_top + self.shake_offset
    }

    pub fn on_screen_resize(&mut self) {
        self.zoom_level = self.zoom_level.clamp(max_zoom_out_level(), 1.0);
        self.constrain_to_map();
    }

    pub fn zoom(&mut self, delta: f32, origin_screen_xy: Xy<Px>) {
        self.constrain_to_map();

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
        let screen_tiles = Xy::new(
            screen_wh.width.as_f32() / TILE_PX_SIZE.width.as_f32(),
            screen_wh.height.as_f32() / TILE_PX_SIZE.height.as_f32(),
        );

        let clamp_or_center = |value: f32, min: f32, max: f32| {
            if min <= max {
                value.clamp(min, max)
            } else {
                (min + max) / 2.0
            }
        };

        self.left_top.x = clamp_or_center(
            self.left_top.x,
            -MAP_OUTSIDE_MARGIN_TILES,
            MAP_SIZE.width as f32 + MAP_OUTSIDE_MARGIN_TILES - screen_tiles.x,
        );
        self.left_top.y = clamp_or_center(
            self.left_top.y,
            -MAP_OUTSIDE_MARGIN_TILES,
            MAP_SIZE.height as f32 + MAP_OUTSIDE_MARGIN_TILES - screen_tiles.y,
        );
    }
}

fn max_zoom_out_level() -> f32 {
    let screen_wh = screen::size().map(|x| x.as_i32() as f32);
    let map_px = Xy::new(
        MAP_SIZE.width as f32 * TILE_PX_SIZE.width.as_f32(),
        MAP_SIZE.height as f32 * TILE_PX_SIZE.height.as_f32(),
    );
    let margin_px = MAP_OUTSIDE_MARGIN_TILES * TILE_PX_SIZE.width.as_f32();
    let max_content_width = map_px.x + 2.0 * margin_px;
    let max_content_height = map_px.y + 2.0 * margin_px;
    (screen_wh.width / max_content_width).max(screen_wh.height / max_content_height)
}
