use namui::{RectFill, RectParam, RectRound, RectStroke, RectStyle, RenderingTree, Wh};

use super::SequenceListView;

impl SequenceListView {
    pub fn render_button_background(&self, wh: Wh<f32>) -> RenderingTree {
        namui::rect(RectParam {
            x: 0.0,
            y: 0.0,
            width: wh.width,
            height: wh.height,
            style: RectStyle {
                stroke: Some(RectStroke {
                    border_position: namui::BorderPosition::Inside,
                    color: namui::Color::from_u8(228, 241, 254, 255),
                    width: 1.0,
                }),
                fill: Some(RectFill {
                    color: namui::Color::from_u8(107, 185, 240, 255),
                }),
                round: Some(RectRound { radius: 4.0 }),
            },
        })
    }
}
