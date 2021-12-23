use crate::{draw::ImageDrawCommand, *};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum ImageFit {
    Fill,
    Contain,
    Cover,
    ScaleDown,
    None,
}

pub struct ImageStyle {
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}

pub struct ImageParam {
    pub xywh: XywhRect<f32>,
    pub url: String,
    pub style: ImageStyle,
}

pub fn image(
    ImageParam {
        url,
        xywh,
        style,
    }: ImageParam,
) -> RenderingTree {
    let image_draw_command = ImageDrawCommand {
        url,
        xywh,
        fit: style.fit,
        paint: style.paint,
    };
    RenderingTree::Node(RenderingData {
        draw_calls: vec![DrawCall {
            commands: vec![DrawCommand::Image(image_draw_command)],
        }],
        ..Default::default()
    })
}
