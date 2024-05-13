use crate::*;

pub struct ImageStyle {
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}

pub struct ImageParam {
    pub rect: Rect<Px>,
    pub image: Image,
    pub style: ImageStyle,
}

pub fn image(ImageParam { image, rect, style }: ImageParam) -> RenderingTree {
    RenderingTree::Node(DrawCommand::Image {
        command: ImageDrawCommand {
            image,
            rect,
            fit: style.fit,
            paint: style.paint,
        }
        .into(),
    })
}
