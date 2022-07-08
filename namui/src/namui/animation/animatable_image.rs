use super::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatableImage {
    pub image_source_url: Option<Url>,
    pub x: KeyframeGraph<Px>,
    pub y: KeyframeGraph<Px>,
    pub width_percent: KeyframeGraph<Percent>,
    pub height_percent: KeyframeGraph<Percent>,
    pub rotation_angle: KeyframeGraph<Angle>,
    pub opacity: KeyframeGraph<OneZero>,
    pub anchor_percent_xy: Xy<Percent>,
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
            anchor_percent_xy: Xy {
                x: Percent::from_percent(50.0),
                y: Percent::from_percent(50.0),
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
    pub fn get_image_px_wh(&self, time: Time) -> Option<Wh<Px>> {
        let width_percent = self.width_percent.get_value(time)?;
        let height_percent = self.height_percent.get_value(time)?;

        self.image_source_url
            .as_ref()
            .and_then(|image_source_url| crate::system::image::try_load(image_source_url))
            .and_then(|image| {
                let size = image.size();
                Some(Wh {
                    width: size.width * width_percent,
                    height: size.height * height_percent,
                })
            })
    }
    pub fn get_anchor_px_wh(&self, playback_time: Time) -> Option<Xy<Px>> {
        self.get_image_px_wh(playback_time).and_then(|image_wh| {
            Some(Xy {
                x: image_wh.width * self.anchor_percent_xy.x,
                y: image_wh.height * self.anchor_percent_xy.y,
            })
        })
    }
    pub fn get_keyframe_infos(&self) -> Vec<KeyframeInfo> {
        get_keyframe_info(&self.x, KeyframeType::X)
            .into_iter()
            .chain(get_keyframe_info(&self.y, KeyframeType::Y))
            .chain(get_keyframe_info(
                &self.width_percent,
                KeyframeType::WidthPercent,
            ))
            .chain(get_keyframe_info(
                &self.height_percent,
                KeyframeType::HeightPercent,
            ))
            .chain(get_keyframe_info(
                &self.rotation_angle,
                KeyframeType::RotationAngle,
            ))
            .chain(get_keyframe_info(&self.opacity, KeyframeType::Opacity))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct KeyframeInfo {
    pub keyframe_type: KeyframeType,
    pub id: String,
    pub time: Time,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyframeType {
    X,
    Y,
    WidthPercent,
    HeightPercent,
    RotationAngle,
    Opacity,
}

fn get_keyframe_info<T: KeyframeValue + Clone>(
    graph: &KeyframeGraph<T>,
    keyframe_type: KeyframeType,
) -> Vec<KeyframeInfo> {
    graph
        .get_points_with_lines()
        .into_iter()
        .map(|(point, _)| KeyframeInfo {
            keyframe_type,
            id: point.id().to_string(),
            time: point.time,
        })
        .collect()
}

impl<T> KeyframeValue for T
where
    T: std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T> + Copy,
{
    fn interpolate(&self, next: &Self, ratio: f32) -> Self {
        *self * (1.0 - ratio) + *next * ratio
    }
}

impl Animate for AnimatableImage {
    fn render(&self, time: Time) -> RenderingTree {
        try_render(|| {
            let opacity = self.opacity.get_value(time)?.as_f32();
            if opacity <= 0.0 {
                return None;
            }
            let angle = self.rotation_angle.get_value(time)?;
            let x = self.x.get_value(time)?;
            let y = self.y.get_value(time)?;
            let source_url = self.image_source_url.as_ref()?.clone();

            let image = crate::system::image::try_load(&source_url)?;

            let image_wh = self.get_image_px_wh(time)?;
            let anchor_xy = self.get_anchor_px_wh(time)?;

            let image_rendering_tree = namui::image(ImageParam {
                rect: Rect::from_xy_wh(Xy::single(px(0.0)), image_wh),
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint_builder: None,
                },
                source: ImageSource::Image(image),
            });
            let transformed_image = namui::translate(
                x,
                y,
                namui::rotate(
                    angle,
                    namui::translate(-anchor_xy.x, -anchor_xy.y, image_rendering_tree),
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
            KeyframePoint::new(Time::Ms(0.0), OneZero::from(0.0)),
            KeyframeLine::Linear,
        );
        graph.put(
            KeyframePoint::new(
                Time::Ms(10.0),
                OneZero::from(100.0), // become 1.0
            ),
            KeyframeLine::Linear,
        );
        graph.put(
            KeyframePoint::new(Time::Ms(20.0), OneZero::from(0.5)),
            KeyframeLine::Linear,
        );
        for time in 0..10 {
            let value = graph.get_value(Time::Ms(time as f32));
            assert_eq!(value, Some((time as f32 / 10.0).into()));
        }
        for time in 10..20 {
            let value = graph.get_value(Time::Ms(time as f32));
            assert!(approx_eq!(
                f32,
                value.unwrap().into(),
                1.0 - (time - 10) as f32 / 20.0,
                ulps = 2
            ));
        }
    }
}
