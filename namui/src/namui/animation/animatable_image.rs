use super::*;
use crate::*;

pub struct AnimatableImage {
    pub image_source_url: String,
    pub x: KeyframeGraph<PixelSize>,
    pub y: KeyframeGraph<PixelSize>,
    pub width: KeyframeGraph<PixelSize>,
    pub height: KeyframeGraph<PixelSize>,
    pub rotation_angle: KeyframeGraph<Angle>,
    pub opacity: KeyframeGraph<OneZero>,
}

impl Animate for AnimatableImage {
    fn render(&self, time: &Time) -> RenderingTree {
        try_render! {
            let opacity: f32 = self.opacity.get_value(time)?.into();
            if opacity <= 0.0 {
                return None;
            }

            Some(namui::image(ImageParam {
                xywh: XywhRect {
                    x: self.x.get_value(time)?.into(),
                    y: self.y.get_value(time)?.into(),
                    width: self.width.get_value(time)?.into(),
                    height: self.height.get_value(time)?.into(),
                },
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint_builder: None,
                },
                source: ImageSource::Url(self.image_source_url.clone()),
            }))
        }
    }
}
