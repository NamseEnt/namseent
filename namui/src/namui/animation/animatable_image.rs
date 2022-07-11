use super::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatableImage {
    pub image_source_url: Option<Url>,
    pub anchor_percent_xy: Xy<Percent>,
    pub image_keyframe_graph: ImageKeyframeGraph,
}
impl AnimatableImage {
    pub fn new() -> Self {
        Self {
            image_source_url: None,
            anchor_percent_xy: Xy {
                x: Percent::from_percent(50.0_f32),
                y: Percent::from_percent(50.0_f32),
            },
            image_keyframe_graph: ImageKeyframeGraph::new(),
        }
    }
    pub fn get_visible_time_range(&self) -> Option<(Time, Time)> {
        if self.image_source_url.is_none() {
            return None;
        }

        let start_time = self.image_keyframe_graph.get_first_point()?.time;
        let end_time = self.image_keyframe_graph.get_last_point()?.time;

        Some((start_time, end_time))
    }
    pub fn get_image_px_wh(&self, time: Time) -> Option<Wh<Px>> {
        let image_keyframe = self.image_keyframe_graph.get_value(time)?;

        self.image_source_url
            .as_ref()
            .and_then(|image_source_url| crate::system::image::try_load(image_source_url))
            .and_then(|image| {
                let size = image.size();
                Some(Wh {
                    width: size.width * image_keyframe.width_percent,
                    height: size.height * image_keyframe.height_percent,
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
    // pub fn get_keyframe_infos(&self) -> Vec<KeyframeInfo> {
    //     get_keyframe_info(&self.x, KeyframeType::X)
    //         .into_iter()
    //         .chain(get_keyframe_info(&self.y, KeyframeType::Y))
    //         .chain(get_keyframe_info(
    //             &self.width_percent,
    //             KeyframeType::WidthPercent,
    //         ))
    //         .chain(get_keyframe_info(
    //             &self.height_percent,
    //             KeyframeType::HeightPercent,
    //         ))
    //         .chain(get_keyframe_info(
    //             &self.rotation_angle,
    //             KeyframeType::RotationAngle,
    //         ))
    //         .chain(get_keyframe_info(&self.opacity, KeyframeType::Opacity))
    //         .collect()
    // }
}

impl Animate for AnimatableImage {
    fn render(&self, time: Time) -> RenderingTree {
        try_render(|| {
            let image_keyframe = self.image_keyframe_graph.get_value(time)?;
            let opacity = image_keyframe.opacity.as_f32();
            if opacity <= 0.0 {
                return None;
            }
            let x = image_keyframe.x;
            let y = image_keyframe.y;
            let source_url = self.image_source_url.as_ref()?.clone();

            let image = crate::system::image::try_load(&source_url)?;

            let image_wh = self.get_image_px_wh(time)?;
            let anchor_xy = self.get_anchor_px_wh(time)?;

            let image_rendering_tree = namui::image(ImageParam {
                rect: Rect::Xywh {
                    x: px(0.0),
                    y: px(0.0),
                    width: image_wh.width,
                    height: image_wh.height,
                },
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
                    image_keyframe.rotation_angle,
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

    struct LinearKeyframeLine {}

    impl KeyframeValue<LinearKeyframeLine> for OneZero {
        fn interpolate(&self, next: &Self, time_ratio: f32, _line: &LinearKeyframeLine) -> Self {
            let one_zero =
                OneZero::from(self.as_f32() * (1.0 - time_ratio) + next.as_f32() * time_ratio);
            one_zero
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn one_zero_should_be_interpolated() {
        let mut graph = KeyframeGraph::new();
        graph.put(
            KeyframePoint::new(Time::Ms(0.0), OneZero::from(0.0)),
            LinearKeyframeLine {},
        );
        graph.put(
            KeyframePoint::new(
                Time::Ms(10.0),
                OneZero::from(100.0), // become 1.0
            ),
            LinearKeyframeLine {},
        );
        graph.put(
            KeyframePoint::new(Time::Ms(20.0), OneZero::from(0.5)),
            LinearKeyframeLine {},
        );
        for time in 0..10 {
            let value = graph.get_value(Time::Ms(time as f32));
            assert_eq!(value, Some((time as f32 / 10.0).into()));
        }
        for time in 10..20 {
            let value = graph.get_value(Time::Ms(time as f32));
            assert!(approx_eq!(
                f32,
                value.unwrap().as_f32(),
                1.0 - (time - 10) as f32 / 20.0,
                ulps = 2
            ));
        }
    }
}
