use super::*;

pub struct Camera {
    pub left_top: MapCoordF32,
    pub zoom_level: f32,
}
impl Camera {
    pub fn new() -> Self {
        Self {
            left_top: MapCoordF32::new(0.0, 0.0),
            zoom_level: 1.0,
        }
    }
    pub fn zoom(&mut self, delta: f32) {
        self.zoom_level += delta;
        self.zoom_level = self.zoom_level.clamp(max_zoom_out_level(), 1.0);
    }
    pub fn move_by(&mut self, screen_px_xy: Xy<Px>) {
        let px_xy_on_1_0 = screen_px_xy / self.zoom_level;
        self.left_top += Xy::new(
            px_xy_on_1_0.x / TILE_PX_SIZE.width,
            px_xy_on_1_0.y / TILE_PX_SIZE.height,
        )
    }
}

fn max_zoom_out_level() -> f32 {
    let screen_wh = screen::size().map(|x| x.as_i32() as f32);
    let shorter_side = screen_wh.width.min(screen_wh.height) as f32;
    shorter_side / (MAP_SIZE.width as f32 * TILE_PX_SIZE.width.as_f32())
}
