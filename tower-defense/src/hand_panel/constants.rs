use crate::hand::HAND_WH;
use namui::*;

pub(super) const CONTAINER_PADDING: Px = px(8.0);
pub(super) const PANEL_PADDING: Px = px(24.0);
pub(super) const BOTTOM_OUTSIDE_HEIGHT: Px = px(24.0);
pub(super) const PAPER_HEIGHT: Px = px(176.0);

pub(super) const PREVIEW_WIDTH: Px = px(160.0);
pub(super) const PREVIEW_HEIGHT: Px = px(160.0);
pub(super) const PREVIEW_RIGHT_OVERLAP: Px = px(12.0);

pub(super) fn panel_width() -> Px {
    HAND_WH.width + (PANEL_PADDING + CONTAINER_PADDING) * 2.0
}
