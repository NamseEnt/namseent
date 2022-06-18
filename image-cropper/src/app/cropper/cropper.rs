use super::{canvas::Canvas, render_app_bar::render_app_bar};
use crate::app::cropper::canvas::CanvasProps;
use namui::{render, translate, Image, RenderingTree, Wh, XywhRect};
use std::sync::Arc;

pub struct CropperProps {
    pub xywh: XywhRect<f32>,
}

pub struct Cropper {
    canvas: Canvas,
}
impl Cropper {
    pub fn new(image: Arc<Image>) -> Self {
        Self {
            canvas: Canvas::new(image.clone()),
        }
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.canvas.update(event);
    }

    pub fn render(&self, props: CropperProps) -> RenderingTree {
        const APP_BAR_HEIGHT: f32 = 48.0;

        render([
            render_app_bar(Wh {
                width: props.xywh.width,
                height: APP_BAR_HEIGHT,
            }),
            translate(
                0.0,
                APP_BAR_HEIGHT,
                self.canvas.render(CanvasProps {
                    wh: Wh {
                        width: props.xywh.width,
                        height: props.xywh.height - APP_BAR_HEIGHT,
                    },
                }),
            ),
        ])
    }
}
