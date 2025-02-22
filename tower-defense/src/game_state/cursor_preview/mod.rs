mod tower;

use super::tower::TowerTemplate;
use crate::MapCoordF32;
use namui::*;
use tower::TowerCursorPreview;

pub struct CursorPreview {
    pub kind: PreviewKind,
    pub map_coord: MapCoordF32,
}
impl CursorPreview {
    pub fn should_update_position(&self) -> bool {
        match self.kind {
            PreviewKind::None => false,
            PreviewKind::PlacingTower { .. } => true,
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
    fn render(self, ctx: &RenderCtx) {
        let CursorPreview { kind, map_coord } = self.inner;

        match kind {
            PreviewKind::None => {}
            PreviewKind::PlacingTower {
                tower_template,
                placing_tower_slot_index,
            } => {
                ctx.add(TowerCursorPreview {
                    tower_template,
                    map_coord: *map_coord,
                    placing_tower_slot_index: *placing_tower_slot_index,
                });
            }
        }
    }
}

#[derive(Clone, Default)]
pub enum PreviewKind {
    #[default]
    None,
    PlacingTower {
        tower_template: TowerTemplate,
        placing_tower_slot_index: usize,
    },
}
