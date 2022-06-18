use super::SelectionTrait;
use namui::{BorderPosition, Color, RectParam, RectStroke, RectStyle, Xy, XywhRect};

#[derive(Clone)]
pub struct RectSelection {
    xywh: XywhRect<f32>,
}
impl RectSelection {
    pub fn new(xywh: XywhRect<f32>) -> Self {
        Self { xywh }
    }
}
impl SelectionTrait for RectSelection {
    fn render(&self, scale: f32) -> namui::RenderingTree {
        namui::rect(RectParam {
            x: self.xywh.x * scale,
            y: self.xywh.y * scale,
            width: self.xywh.width * scale,
            height: self.xywh.height * scale,
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: Color::grayscale_f01(0.5),
                    width: 1.0,
                    border_position: BorderPosition::Inside,
                }),
                ..Default::default()
            },
        })
    }

    fn get_polygon(&self) -> Vec<namui::Xy<f32>> {
        vec![
            Xy {
                x: self.xywh.x,
                y: self.xywh.y,
            },
            Xy {
                x: self.xywh.x + self.xywh.width,
                y: self.xywh.y,
            },
            Xy {
                x: self.xywh.x + self.xywh.width,
                y: self.xywh.y + self.xywh.height,
            },
            Xy {
                x: self.xywh.x,
                y: self.xywh.y + self.xywh.height,
            },
        ]
    }
}
