use super::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatableImage {
    pub image_source_url: Option<Url>,
    pub x: KeyframeGraph<PixelSize>,
    pub y: KeyframeGraph<PixelSize>,
    pub width: KeyframeGraph<Percent>,
    pub height: KeyframeGraph<Percent>,
    pub rotation_angle: KeyframeGraph<Degree>,
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
    pub fn get_visible_time_range(&self) -> Option<(Time, Time)> {
        if self.image_source_url.is_none() {
            return None;
        }

        let start_time = [
            self.x.get_first_point().map(|point| point.time),
            self.y.get_first_point().map(|point| point.time),
            self.width.get_first_point().map(|point| point.time),
            self.height.get_first_point().map(|point| point.time),
            self.rotation_angle
                .get_first_point()
                .map(|point| point.time),
            self.opacity.get_first_point().map(|point| point.time),
        ]
        .into_iter()
        .filter_map(|time| time)
        .min();

        let end_time = [
            self.x.get_last_point().map(|point| point.time),
            self.y.get_last_point().map(|point| point.time),
            self.width.get_last_point().map(|point| point.time),
            self.height.get_last_point().map(|point| point.time),
            self.rotation_angle.get_last_point().map(|point| point.time),
            self.opacity.get_last_point().map(|point| point.time),
        ]
        .into_iter()
        .filter_map(|time| time)
        .max();

        if let Some(start_time) = start_time {
            if let Some(end_time) = end_time {
                return Some((start_time, end_time));
            }
        }
        None
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
impl KeyframeValue for Percent {
    fn interpolate(&self, next: &Self, ratio: f32) -> Self {
        Self::new(self.0 * (1.0 - ratio) + next.0 * ratio)
    }
    fn unit() -> &'static str {
        "%"
    }
}
impl KeyframeValue for Degree {
    fn interpolate(&self, next: &Self, ratio: f32) -> Self {
        self * (1.0 - ratio) + next * ratio
    }
    fn unit() -> &'static str {
        "°"
    }
}
impl KeyframeValue for OneZero {
    fn interpolate(&self, next: &Self, ratio: f32) -> Self {
        Self::from(self.0 * (1.0 - ratio) + next.0 * ratio)
    }
    fn unit() -> &'static str {
        ""
    }
}

impl Animate for AnimatableImage {
    fn render(&self, time: Time) -> RenderingTree {
        try_render(|| {
            let opacity: f32 = self.opacity.get_value(time)?.into();
            if opacity <= 0.0 {
                return None;
            }
            let ccw_radian = -self.rotation_angle.get_value(time)?.to_radian();
            let x = self.x.get_value(time)?;
            let y = self.y.get_value(time)?;
            let width = self.width.get_value(time)?;
            let height = self.height.get_value(time)?;
            let source_url = self.image_source_url.as_ref()?.clone();

            let managers = namui::managers();
            let image = managers.image_manager.try_load(&source_url)?;
            let image_size = image.size();

            Some(namui::rotate(
                ccw_radian.into(),
                namui::image(ImageParam {
                    xywh: XywhRect {
                        x: x.into(),
                        y: y.into(),
                        width: (width * image_size.width).into(),
                        height: (height * image_size.height).into(),
                    },
                    style: ImageStyle {
                        fit: ImageFit::Fill,
                        paint_builder: None,
                    },
                    source: ImageSource::Image(image),
                }),
            ))
        })
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
            KeyframePoint::new(Time::from_ms(0.0), OneZero::from(0.0)),
            KeyframeLine::Linear,
        );
        graph.put(
            KeyframePoint::new(
                Time::from_ms(10.0),
                OneZero::from(100.0), // become 1.0
            ),
            KeyframeLine::Linear,
        );
        graph.put(
            KeyframePoint::new(Time::from_ms(20.0), OneZero::from(0.5)),
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
