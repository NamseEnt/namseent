mod item;
mod tower;

use super::{item::Item, tower::TowerTemplate};
use crate::{MapCoordF32, game_state::hand::HandSlotId};
use item::ItemCursorPreview;
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
            PreviewKind::Item { .. } => true,
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
                placing_tower_slot_id,
            } => {
                ctx.add(TowerCursorPreview {
                    tower_template,
                    map_coord: *map_coord,
                    placing_tower_slot_id: *placing_tower_slot_id,
                });
            }
            PreviewKind::Item { item, item_index } => {
                ctx.add(ItemCursorPreview {
                    item,
                    item_index: *item_index,
                    map_coord: *map_coord,
                });
            }
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum PreviewKind {
    #[default]
    None,
    PlacingTower {
        tower_template: TowerTemplate,
        placing_tower_slot_id: HandSlotId,
    },
    Item {
        item: Item,
        item_index: usize,
    },
}
