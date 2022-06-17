use super::render_back_button::render_back_button;
use namui::{image, render, Image, ImageFit, ImageParam, ImageStyle, RenderingTree, Wh, XywhRect};
use std::sync::Arc;

pub struct CropperProps {
    pub xywh: XywhRect<f32>,
}

pub struct Cropper {
    image: Arc<Image>,
}
impl Cropper {
    pub fn new(image: Arc<Image>) -> Self {
        Self { image }
    }

    pub fn update(&mut self, _event: &dyn std::any::Any) {}

    pub fn render(&self, props: CropperProps) -> RenderingTree {
        render([
            image(ImageParam {
                xywh: props.xywh,
                source: namui::ImageSource::Image(self.image.clone()),
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint_builder: None,
                },
            }),
            render_back_button(Wh {
                width: 128.0,
                height: 32.0,
            }),
        ])
    }
}
