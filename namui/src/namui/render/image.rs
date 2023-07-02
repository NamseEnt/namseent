use crate::{draw::ImageDrawCommand, *};
use serde::Serialize;
use std::sync::Arc;

/// Example: https://developer.mozilla.org/ko/docs/Web/CSS/object-fit
#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub enum ImageFit {
    /// The replaced content is sized to fill the element's content box.
    /// The entire object will completely fill the box.
    /// If the object's aspect ratio does not match the aspect ratio of its box,
    /// then the object will be stretched to fit.
    Fill,
    /// The replaced content is scaled to maintain its aspect ratio while fitting within the element's content box.
    /// The entire object is made to fill the box, while preserving its aspect ratio, so the object will be letterboxed
    /// if its aspect ratio does not match the aspect ratio of the box.
    Contain,
    /// The replaced content is sized to maintain its aspect ratio while filling the element's entire content box.
    /// If the object's aspect ratio does not match the aspect ratio of its box, then the object will be clipped to fit.
    Cover,
    /// The content is sized as if `none` or `contain` were specified, whichever would result in a smaller concrete object size.
    ScaleDown,
    /// The replaced content is not resized.
    None,
}

pub struct ImageStyle {
    pub fit: ImageFit,
    pub paint_builder: Option<PaintBuilder>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum ImageSource {
    Url(Url),
    Image(Arc<Image>),
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
