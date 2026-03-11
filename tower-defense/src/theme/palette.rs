#![allow(dead_code)]
use namui::*;

pub const PRIMARY: Color = Color::from_u8(147, 180, 211, 255);
pub const ON_PRIMARY: Color = Color::from_u8(37, 26, 31, 255);
pub const SECONDARY: Color = Color::from_u8(224, 113, 111, 255);
pub const ON_SECONDARY: Color = Color::from_u8(72, 0, 1, 255);

pub const SURFACE: Color = Color::from_u8(128, 105, 76, 255);
pub const SURFACE_CONTAINER_LOWEST: Color = Color::from_u8(171, 128, 79, 255);
pub const SURFACE_CONTAINER_LOW: Color = Color::from_u8(208, 175, 124, 255);
pub const SURFACE_CONTAINER: Color = Color::from_u8(223, 166, 155, 255);
pub const SURFACE_CONTAINER_HIGH: Color = Color::from_u8(230, 216, 185, 255);
pub const SURFACE_CONTAINER_HIGHEST: Color = Color::from_u8(239, 221, 191, 255);
pub const ON_SURFACE: Color = Color::from_u8(37, 26, 31, 255);
pub const ON_SURFACE_VARIANT: Color = Color::from_u8(72, 0, 1, 255);
pub const OUTLINE: Color = Color::from_u8(132, 108, 80, 255);

pub const COMMON: Color = Color::from_u8(234, 242, 215, 255);
pub const RARE: Color = Color::from_u8(3, 138, 255, 255);
pub const EPIC: Color = Color::from_u8(191, 85, 236, 255);
pub const LEGENDARY: Color = Color::from_u8(216, 250, 8, 255);

pub const RED: Color = Color::from_u8(244, 67, 54, 255);
pub const BLUE: Color = Color::from_u8(33, 150, 243, 255);
pub const YELLOW: Color = Color::from_u8(255, 193, 7, 255);

pub const WHITE: Color = Color::from_u8(255, 255, 255, 255);
pub const BLACK: Color = Color::from_u8(0, 0, 0, 255);

pub const DISABLED_CONTAINER: Color = Color::from_u8(115, 85, 80, 255);
pub const ON_DISABLED_CONTAINER: Color = Color::from_u8(141, 121, 95, 255);

pub const ROUND: Px = px(8.);

pub fn modal_box_style() -> RectStyle {
    RectStyle {
        stroke: None,
        fill: Some(RectFill { color: SURFACE }),
        round: Some(RectRound { radius: ROUND }),
    }
}

pub fn title_background_style() -> RectStyle {
    RectStyle {
        stroke: None,
        fill: Some(RectFill {
            color: SURFACE_CONTAINER,
        }),
        round: Some(RectRound { radius: ROUND }),
    }
}

pub fn container_box_style() -> RectStyle {
    title_background_style()
}
