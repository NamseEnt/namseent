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

pub enum ImageSource {
    Image { image: Image },
    Url { url: String },
}

pub struct ImageRender {
    pub rect: Rect<Px>,
    pub source: ImageSource,
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}

impl StaticType for ImageRender {}

impl Component for ImageRender {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            rect,
            source,
            fit,
            paint,
        } = self;
        match source {
            ImageSource::Image { image } => {
                ctx.add(crate::image(crate::ImageParam {
                    rect,
                    image,
                    style: ImageStyle { fit, paint },
                }));
            }
            ImageSource::Url { url } => {
                let image = ctx.image(url);

                let Some(Ok(image)) = image.as_ref() else {
                    return;
                };

                ctx.add(crate::image(crate::ImageParam {
                    rect,
                    image: image.clone(),
                    style: ImageStyle { fit, paint },
                }));
            }
        }
    }
}
