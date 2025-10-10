use crate::*;

#[derive(Clone)]
pub struct ImageStyle {
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}

#[derive(Clone)]
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
    ResourceLocation { resource_location: ResourceLocation },
}

pub struct ImageRender {
    pub rect: Rect<Px>,
    pub source: ImageSource,
    pub fit: ImageFit,
    pub paint: Option<Paint>,
}

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
            ImageSource::ResourceLocation { resource_location } => {
                let image_result = ctx.image(resource_location);
                let image_guard = image_result.lock().unwrap();

                let Some(Ok(image)) = image_guard.as_ref() else {
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
