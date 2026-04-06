use namui::*;

pub const MODAL_WIDTH: Px = px(540.0);
pub const MODAL_HEIGHT: Px = px(480.0);
pub const PADDING: Px = px(8.0);
pub const CARD_GAP: Px = px(24.0);
pub const TREASURE_BG_PADDING: Px = px(192.0);
pub const TREASURE_HALO_PADDING: Px = px(16.0);

pub mod card;
pub mod ui;

pub use ui::TreasureSelectionUi;
