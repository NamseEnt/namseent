mod animation;
mod image;
mod range;
mod sprite;
mod tower;

pub(crate) use animation::Animation;
pub use animation::{AnimationKind, tower_animation_tick};
pub use image::TowerImage;
pub(crate) use range::TowerAttackRange;
pub use sprite::TowerSpriteWithOverlay;
pub use tower::RenderTower;

pub const TOWER_OVERLAY_SUIT_X_RATIO: f32 = 0.3;
pub const TOWER_OVERLAY_RANK_X_RATIO: f32 = 0.7;
pub const TOWER_OVERLAY_SIDE_Y_RATIO: f32 = 0.75;
pub const TOWER_OVERLAY_ICON_SCALE: f32 = 1.0;
pub const TOWER_OVERLAY_ROTATION_DEG: f32 = -12.0;
