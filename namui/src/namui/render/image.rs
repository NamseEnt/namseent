use std::sync::Arc;

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

#[derive(Debug, Serialize)]
pub enum ImageSource {
    Url(String),
    #[serde(skip_serializing)]
    Image(Arc<Image>),
}

pub struct ImageParam {
    pub xywh: XywhRect<f32>,
    pub source: ImageSource,
    pub style: ImageStyle,
}

pub fn image(
    ImageParam {
        source,
        xywh,
        style,
    }: ImageParam,
) -> RenderingTree {
    let image_draw_command = ImageDrawCommand {
        source,
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
