use namui::*;

pub(super) const SHOP_PANEL_HEIGHT: Px = px(480.0);
pub(super) const PAPER_WIDTH: Px = px(960.0);

pub(super) const BG_HEIGHT: Px = px(240.0);

pub(super) const OUTSIDE_HEIGHT: Px = px(24.0);

pub(super) const PANEL_PADDING: Px = px(24.0);

pub const PADDING: Px = px(4.0);
pub const SHOP_SLOT_WIDTH: Px = px(136.0);
pub const SHOP_SLOT_HEIGHT: Px = px(176.0);

// pub(super) const VOYAGER_WIDTH: Px = px(320.0);
// pub(super) const VOYAGER_HEIGHT: Px = px(320.0);
// pub(super) const VOYAGER_ANIM_PERIOD: std::time::Duration = std::time::Duration::from_millis(660);

#[inline]
pub(super) fn panel_width() -> Px {
    PAPER_WIDTH
}

#[inline]
pub(super) fn shop_panel_wh() -> Wh<Px> {
    Wh::new(panel_width(), SHOP_PANEL_HEIGHT + OUTSIDE_HEIGHT)
}
