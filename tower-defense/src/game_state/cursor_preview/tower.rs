use crate::{game_state::tower::TowerTemplate, MapCoord};
use namui::*;

pub(super) struct TowerCursorPreview<'a> {
    pub tower_template: &'a TowerTemplate,
    pub map_coord: MapCoord,
}
impl Component for TowerCursorPreview<'_> {
    fn render(self, ctx: &namui::RenderCtx) {}
}
