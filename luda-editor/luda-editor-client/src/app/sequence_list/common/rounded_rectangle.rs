use crate::app::sequence_list::RECT_RADIUS;
use namui::prelude::*;
use std::convert;

pub enum RoundedRectangleColor {
    DarkGray,
    Gray,
    LightGray,
    Blue,
    White,
}

pub fn render_rounded_rectangle(wh: Wh<Px>, color: RoundedRectangleColor) -> RenderingTree {
    namui::rect(RectParam {
        rect: Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: wh.width,
            height: wh.height,
        },
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

impl convert::Into<Color> for RoundedRectangleColor {
    fn into(self) -> Color {
        match self {
            RoundedRectangleColor::DarkGray => Color::grayscale_f01(0.3),
            RoundedRectangleColor::Gray => Color::grayscale_f01(0.5),
            RoundedRectangleColor::LightGray => Color::grayscale_f01(0.8),
            RoundedRectangleColor::Blue => Color::from_u8(107, 185, 240, 255),
            RoundedRectangleColor::White => Color::WHITE,
        }
    }
}
