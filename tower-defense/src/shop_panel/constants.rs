use namui::*;

pub(super) const TOP_BAR_HEIGHT: Px = px(48.0);

pub(super) const STICKY_WIDTH: Px = px(128.0);
pub(super) const STICKY_HEIGHT: Px = px(48.0);
pub(super) const STICKY_VISIBLE_HEIGHT: Px = px(24.0);

pub(super) const PAPER_HEIGHT: Px = px(480.0);
pub(super) const PAPER_WIDTH: Px = px(960.0);

pub(super) const BG_HEIGHT: Px = px(240.0);

pub(super) const ACTION_MARGIN_Y: Px = px(24.0);

pub(super) const BUTTON_SPACING: Px = px(16.0);

pub(super) fn action_area_width() -> Px {
    ACTION_WIDTH * 2.0 + INNER_PADDING * 2.0
}

pub(super) const TOP_OUTSIDE_HEIGHT: Px = px(24.0);

pub(super) const ACTION_HEIGHT: Px = px(48.0);
pub(super) const ACTION_WIDTH: Px = px(160.0);

pub(super) const PANEL_PADDING: Px = px(24.0);
pub(super) const INNER_PADDING: Px = px(8.0);

pub const PADDING: Px = px(4.0);
pub const SOLD_OUT_HEIGHT: Px = px(36.0);
pub const SHOP_SLOT_WIDTH: Px = px(240.0);

pub(super) fn panel_width() -> Px {
    PAPER_WIDTH
}

pub(super) fn shop_panel_wh() -> Wh<Px> {
    Wh::new(
        panel_width(),
        STICKY_HEIGHT + PAPER_HEIGHT + TOP_OUTSIDE_HEIGHT,
    )
}
