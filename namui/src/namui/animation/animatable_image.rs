use super::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatableImage {
    pub image_source_url: Option<Url>,
    pub x: KeyframeGraph<PixelSize>,
    pub y: KeyframeGraph<PixelSize>,
    pub width_percent: KeyframeGraph<Percent>,
    pub height_percent: KeyframeGraph<Percent>,
    pub rotation_angle: KeyframeGraph<Degree>,
    pub opacity: KeyframeGraph<OneZero>,
    pub anchor_xy: Xy<Percent>,
}
impl AnimatableImage {
    pub fn new() -> Self {
        Self {
            image_source_url: None,
            x: KeyframeGraph::new(),
            y: KeyframeGraph::new(),
            width_percent: KeyframeGraph::new(),
            height_percent: KeyframeGraph::new(),
            rotation_angle: KeyframeGraph::new(),
            opacity: KeyframeGraph::new(),
            anchor_xy: Xy {
                x: Percent::new(50.0),
                y: Percent::new(50.0),
            },
        }
    }
    pub fn get_visible_time_range(&self) -> Option<(Time, Time)> {
        if self.image_source_url.is_none() {
            return None;
        }

        let start_time = [
            self.x.get_first_point().map(|point| point.time),
            self.y.get_first_point().map(|point| point.time),
            self.width_percent.get_first_point().map(|point| point.time),
            self.height_percent
                .get_first_point()
                .map(|point| point.time),
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
            self.width_percent.get_last_point().map(|point| point.time),
            self.height_percent.get_last_point().map(|point| point.time),
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
    pub fn get_image_pixel_size_wh(&self, time: Time) -> Option<Wh<PixelSize>> {
        let width_percent = self.width_percent.get_value(time)?;
        let height_percent = self.height_percent.get_value(time)?;

        self.image_source_url
            .as_ref()
            .and_then(|image_source_url| crate::system::image::try_load(image_source_url))
            .and_then(|image| {
                let size = image.size();
                Some(Wh {
                    width: (width_percent * size.width).into(),
                    height: (height_percent * size.height).into(),
                })
            })
    }
    pub fn get_anchor_pixel_size_wh(&self, playback_time: Time) -> Option<Xy<PixelSize>> {
        self.get_image_pixel_size_wh(playback_time)
            .and_then(|image_wh| {
                Some(Xy {
                    x: self.anchor_xy.x * image_wh.width,
                    y: self.anchor_xy.y * image_wh.height,
                })
            })
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
            let radian = self.rotation_angle.get_value(time)?.to_radian();
            let x = self.x.get_value(time)?;
            let y = self.y.get_value(time)?;
            let source_url = self.image_source_url.as_ref()?.clone();

            let image = crate::system::image::try_load(&source_url)?;

            let image_wh = self.get_image_pixel_size_wh(time)?;
            let anchor_xy = self.get_anchor_pixel_size_wh(time)?;

            let image_rendering_tree = namui::image(ImageParam {
                xywh: XywhRect {
                    x: 0.0,
                    y: 0.0,
                    width: image_wh.width.into(),
                    height: image_wh.height.into(),
                },
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint_builder: None,
                },
                source: ImageSource::Image(image),
            });
            let transformed_image = namui::translate(
                x.into(),
                y.into(),
                namui::rotate(
                    radian.into(),
                    namui::translate(
                        (-anchor_xy.x).into(),
                        (-anchor_xy.y).into(),
                        image_rendering_tree,
                    ),
                ),
            );

            Some(transformed_image)
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
