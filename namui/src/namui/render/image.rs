use crate::{draw::ImageDrawCommand, file::picker::File, *};
use serde::Serialize;
use std::sync::Arc;

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
    pub paint_builder: Option<PaintBuilder>,
}

#[derive(Debug, Serialize, Clone)]
pub enum ImageSource {
    Url(Url),
    #[serde(skip_serializing)]
    Image(Arc<Image>),
    #[serde(skip_serializing)]
    File(File),
}

pub struct ImageParam {
    pub rect: Rect<Px>,
    pub source: ImageSource,
    pub style: ImageStyle,
}

pub fn image(
    ImageParam {
        source,
        rect,
        style,
    }: ImageParam,
) -> RenderingTree {
    let image_draw_command = ImageDrawCommand {
        source,
        rect,
        fit: style.fit,
        paint_builder: style.paint_builder,
    };
    RenderingTree::Node(RenderingData {
        draw_calls: vec![DrawCall {
            commands: vec![DrawCommand::Image(image_draw_command)],
        }],
        ..Default::default()
    })
}
