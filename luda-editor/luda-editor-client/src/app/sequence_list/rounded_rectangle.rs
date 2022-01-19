use std::convert;

use super::{SequenceList, RECT_RADIUS};
use namui::{Color, RectFill, RectParam, RectRound, RectStyle, RenderingTree, Wh};

pub enum RoundedRectangleColor {
    DarkGray,
    Gray,
    LightGray,
    Blue,
    // Red,
    // White,
}

impl SequenceList {
    pub fn render_rounded_rectangle(
        &self,
        wh: Wh<f32>,
        color: RoundedRectangleColor,
    ) -> RenderingTree {
        namui::rect(RectParam {
            x: 0.0,
            y: 0.0,
            width: wh.width,
            height: wh.height,
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill {
                    color: color.into(),
                }),
                round: Some(RectRound {
                    radius: RECT_RADIUS,
                }),
            },
        })
    }
}

impl convert::Into<Color> for RoundedRectangleColor {
    fn into(self) -> Color {
        match self {
            RoundedRectangleColor::DarkGray => Color::grayscale_f01(0.3),
            RoundedRectangleColor::Gray => Color::grayscale_f01(0.5),
            RoundedRectangleColor::LightGray => Color::grayscale_f01(0.8),
            RoundedRectangleColor::Blue => Color::from_u8(107, 185, 240, 255),
            // RoundedRectangleColor::Red => Color::from_u8(242, 38, 19, 255),
            // RoundedRectangleColor::White => Color::WHITE,
        }
    }
}
