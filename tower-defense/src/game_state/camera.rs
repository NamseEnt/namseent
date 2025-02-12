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
        self.zoom_level = self.zoom_level.max(0.1);
    }
    pub fn move_by(&mut self, screen_px_xy: Xy<Px>) {
        let px_xy_on_1_0 = screen_px_xy / self.zoom_level;
        self.left_top += Xy::new(
            px_xy_on_1_0.x / TILE_PX_SIZE.width,
            px_xy_on_1_0.y / TILE_PX_SIZE.height,
        )
    }
}
