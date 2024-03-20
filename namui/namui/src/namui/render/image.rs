use crate::*;

pub struct ImageStyle {
    pub fit: ImageFit,
    pub paint: Option<Paint>,
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
    RenderingTree::Node(DrawCommand::Image {
        command: ImageDrawCommand {
            source,
            rect,
            fit: style.fit,
            paint: style.paint,
        }
        .into(),
    })
}
