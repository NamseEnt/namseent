use super::*;

pub struct ImageCell {
    image_source: ImageSource,
    on_edit: Option<Box<dyn Fn()>>,
}
pub fn image(image_source: ImageSource) -> ImageCell {
    ImageCell {
        image_source,
        on_edit: None,
    }
}
impl ImageCell {
    pub fn on_edit(self, callback: impl Fn() + 'static) -> Self {
        Self {
            on_edit: Some(Box::new(callback)),
            ..self
        }
    }
}

impl Cell for ImageCell {
    fn render(&self, props: Props) -> RenderingTree {
        namui::image(ImageParam {
            rect: Rect::from_xy_wh(Xy::zero(), props.wh),
            source: self.image_source.clone(),
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint_builder: None,
            },
        })
    }
}

impl Into<Box<dyn Cell>> for ImageCell {
    fn into(self) -> Box<dyn Cell> {
        Box::new(self)
    }
}
