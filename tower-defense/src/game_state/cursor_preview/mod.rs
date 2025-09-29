pub mod tower;

use crate::MapCoordF32;
use namui::*;

pub struct CursorPreview {
    pub kind: PreviewKind,
    pub map_coord: MapCoordF32,
}
impl CursorPreview {
    pub fn should_update_position(&self) -> bool {
        match self.kind {
            PreviewKind::None => false,
        }
    }
    pub fn update_position(&mut self, map_coord: MapCoordF32) {
        self.map_coord = map_coord;
    }
    pub fn render(&self) -> impl Component + '_ {
        RenderCursorPreview { inner: self }
    }
}
impl Default for CursorPreview {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            map_coord: MapCoordF32::new(0., 0.),
        }
    }
}

struct RenderCursorPreview<'a> {
    inner: &'a CursorPreview,
}
impl Component for RenderCursorPreview<'_> {
    fn render(self, _ctx: &RenderCtx) {
        let CursorPreview { kind, .. } = self.inner;

        match kind {
            PreviewKind::None => {}
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum PreviewKind {
    #[default]
    None,
}
