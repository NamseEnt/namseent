use crate::hand::HAND_WH;
use namui::*;

pub(super) const CONTAINER_PADDING: Px = px(8.0);
pub(super) const PANEL_PADDING: Px = px(24.0);
pub(super) const INNER_PADDING: Px = px(8.0);
pub(super) const INTERACTION_CONTAINER_PADDING: Px = px(8.0);
pub(super) const BOTTOM_OUTSIDE_HEIGHT: Px = px(24.0);
pub(super) const STICKY_WIDTH: Px = px(128.0);
pub(super) const STICKY_HEIGHT: Px = px(48.0);
pub(super) const STICKY_VISIBLE_HEIGHT: Px = px(24.0);

pub(super) const STICKY_SHIFT: Px = px(64.0);

pub(super) const PAPER_HEIGHT: Px = px(176.0);
pub(super) const ACTION_WIDTH: Px = px(160.0);

pub(super) fn interaction_width() -> Px {
    ACTION_WIDTH + INTERACTION_CONTAINER_PADDING * 2.0
}

pub(super) fn panel_width() -> Px {
    HAND_WH.width + interaction_width() + PANEL_PADDING * 2.0 + CONTAINER_PADDING * 2.0
}
