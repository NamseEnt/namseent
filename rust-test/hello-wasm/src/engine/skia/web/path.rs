use crate::engine;

use super::*;
pub use base::*;

pub struct Path(pub(crate) CanvasKitPath);
impl Path {
    pub fn new() -> Self {
        Path(CanvasKitPath::new())
    }
    pub fn add_rect(
        &self,
        LtrbRect {
            left,
            top,
            right,
            bottom,
        }: LtrbRect,
    ) {
        let mut array = js_sys::Float32Array::new_with_length(4);
        array.set_index(0, left as f32);
        array.set_index(1, top as f32);
        array.set_index(2, right as f32);
        array.set_index(3, bottom as f32);

        self.0.addRect(array, None);
    }
    pub fn add_rrect(
        &self,
        LtrbRect {
            left,
            top,
            right,
            bottom,
        }: LtrbRect,
        rx: f32,
        ry: f32,
    ) {
        let mut rect = js_sys::Float32Array::new_with_length(4);
        rect.set_index(0, left as f32);
        rect.set_index(1, top as f32);
        rect.set_index(2, right as f32);
        rect.set_index(3, bottom as f32);
        let rrect = canvas_kit().RRectXY(rect, rx, ry);
        self.0.addRRect(rrect, None);
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        engine::log("Dropping Path".to_string());
        self.0.delete();
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path")
    }
}

impl Clone for Path {
    fn clone(&self) -> Self {
        Path(self.0.copy())
    }
}
