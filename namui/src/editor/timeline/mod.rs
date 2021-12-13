use crate::{engine, render};

use super::Clip;

pub struct Timeline {
    pub xywh: engine::XywhRect<f32>,
    pub selected_clip: Option<Clip>,
}

impl engine::Entity for Timeline {
    type RenderingContext = ();

    fn update(&mut self, event: &dyn std::any::Any) {}

    fn render(&self, context: &Self::RenderingContext) -> engine::RenderingTree {
        render![engine::rect(engine::RectParam {
            x: self.xywh.x,
            y: self.xywh.y,
            width: self.xywh.width,
            height: self.xywh.height,
            style: engine::RectStyle {
                fill: Some(engine::RectFill {
                    color: engine::Color::BLACK,
                }),
                ..Default::default()
            },
            ..Default::default()
        })]
    }
}
