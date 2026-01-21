#![allow(dead_code)]
use namui::*;

pub const PRIMARY: Color = Color::from_u8(208, 188, 254, 255);
pub const ON_PRIMARY: Color = Color::from_u8(56, 30, 114, 255);
pub const SECONDARY: Color = Color::from_u8(204, 194, 220, 255);
pub const ON_SECONDARY: Color = Color::from_u8(51, 45, 65, 255);

pub const SURFACE: Color = Color::from_u8(20, 18, 24, 255);
pub const SURFACE_CONTAINER_LOWEST: Color = Color::from_u8(15, 13, 19, 255);
pub const SURFACE_CONTAINER_LOW: Color = Color::from_u8(29, 27, 32, 255);
pub const SURFACE_CONTAINER: Color = Color::from_u8(33, 31, 38, 255);
pub const SURFACE_CONTAINER_HIGH: Color = Color::from_u8(43, 41, 48, 255);
pub const SURFACE_CONTAINER_HIGHEST: Color = Color::from_u8(54, 52, 59, 255);
pub const ON_SURFACE: Color = Color::from_u8(230, 224, 233, 255);
pub const ON_SURFACE_VARIANT: Color = Color::from_u8(202, 196, 208, 255);
pub const OUTLINE: Color = Color::from_u8(147, 143, 153, 255);

pub const COMMON: Color = Color::from_u8(234, 242, 215, 255);
pub const RARE: Color = Color::from_u8(3, 138, 255, 255);
pub const EPIC: Color = Color::from_u8(191, 85, 236, 255);
pub const LEGENDARY: Color = Color::from_u8(216, 250, 8, 255);

pub const RED: Color = Color::from_u8(244, 67, 54, 255);
pub const BLUE: Color = Color::from_u8(33, 150, 243, 255);
pub const YELLOW: Color = Color::from_u8(255, 193, 7, 255);

// Additional colors for buttons
pub const WHITE: Color = Color::from_u8(255, 255, 255, 255);
pub const BLACK: Color = Color::from_u8(0, 0, 0, 255);

// Button disabled states
pub const DISABLED_CONTAINER: Color = Color::from_u8(60, 60, 60, 255);
pub const ON_DISABLED_CONTAINER: Color = Color::from_u8(120, 120, 120, 255);

pub const ROUND: Px = px(8.);

/// 모달 박스 배경 스타일 (SURFACE 색상)
pub fn modal_box_style() -> RectStyle {
    RectStyle {
        stroke: None,
        fill: Some(RectFill { color: SURFACE }),
        round: Some(RectRound { radius: ROUND }),
    }
}

/// 제목 배경 스타일 (SURFACE_CONTAINER 색상)
pub fn title_background_style() -> RectStyle {
    RectStyle {
        stroke: None,
        fill: Some(RectFill {
            color: SURFACE_CONTAINER,
        }),
        round: Some(RectRound { radius: ROUND }),
    }
}

/// 일반 컨테이너 박스 스타일 (SURFACE_CONTAINER 색상)
pub fn container_box_style() -> RectStyle {
    title_background_style()
}
