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
    RenderingTree::Node(RenderingData {
        draw_calls: vec![DrawCall {
            commands: vec![DrawCommand::Image {
                command: ImageDrawCommand {
                    source,
                    rect,
                    fit: style.fit,
                    paint: style.paint,
                },
            }],
        }],
    })
}
