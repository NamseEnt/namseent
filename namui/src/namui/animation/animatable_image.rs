use super::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatableImage {
    pub image_source_url: Option<Url>,
    pub x: KeyframeGraph<PixelSize>,
    pub y: KeyframeGraph<PixelSize>,
    pub width: KeyframeGraph<PixelSize>,
    pub height: KeyframeGraph<PixelSize>,
    pub rotation_angle: KeyframeGraph<Angle>,
    pub opacity: KeyframeGraph<OneZero>,
}
impl AnimatableImage {
    pub fn new() -> Self {
        Self {
            image_source_url: None,
            x: KeyframeGraph::new(),
            y: KeyframeGraph::new(),
            width: KeyframeGraph::new(),
            height: KeyframeGraph::new(),
            rotation_angle: KeyframeGraph::new(),
            opacity: KeyframeGraph::new(),
        }
    }
}

impl KeyframeValue for PixelSize {
    fn interpolate(&self, next: &Self, ratio: f32) -> Self {
        self * (1.0 - ratio) + next * ratio
    }
    fn unit() -> &'static str {
        "px"
    }
}
impl KeyframeValue for Angle {
    fn interpolate(&self, next: &Self, ratio: f32) -> Self {
        self * (1.0 - ratio) + next * ratio
    }
    fn unit() -> &'static str {
        "°"
    }
}
impl KeyframeValue for OneZero {
    fn interpolate(&self, next: &Self, ratio: f32) -> Self {
        Self::new(self.0 * (1.0 - ratio) + next.0 * ratio)
    }
    fn unit() -> &'static str {
        ""
    }
}

impl Animate for AnimatableImage {
    fn render(&self, time: Time) -> RenderingTree {
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
                source: ImageSource::Url(self.image_source_url.as_ref()?.clone()),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn one_zero_should_be_interpolated() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::from_ms(0.0), OneZero::new(0.0)),
            KeyframeLine::Linear,
        );
        graph.put(
            KeyframePoint::new(
                Time::from_ms(10.0),
                OneZero::new(100.0), // become 1.0
            ),
            KeyframeLine::Linear,
        );
        graph.put(
            KeyframePoint::new(Time::from_ms(20.0), OneZero::new(0.5)),
            KeyframeLine::Linear,
        );
        for time in 0..10 {
            let value = graph.get_value(Time::from_ms(time as f32));
            assert_eq!(value, Some((time as f32 / 10.0).into()));
        }
        for time in 10..20 {
            let value = graph.get_value(Time::from_ms(time as f32));
            assert!(approx_eq!(
                f32,
                value.unwrap().into(),
                1.0 - (time - 10) as f32 / 20.0,
                ulps = 2
            ));
        }
    }
}
