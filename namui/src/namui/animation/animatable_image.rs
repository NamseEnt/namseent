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
        let image_renderable = self.image_keyframe_graph.get_value(time)?;
        match image_renderable {
            ImageRenderable::Matrix { matrix, .. } => self
                .image_source_url
                .as_ref()
                .and_then(|image_source_url| crate::system::image::try_load(image_source_url))
                .and_then(|image| {
                    let size = image.size();
                    Some(Wh {
                        width: size.width * Percent::from(matrix.sx()),
                        height: size.height * Percent::from(matrix.sy()),
                    })
                }),
            ImageRenderable::Shader { .. } => None,
        }
    }
    pub fn get_anchor_px_wh(&self, time: Time) -> Option<Xy<Px>> {
        self.get_image_px_wh(time).and_then(|image_wh| {
            Some(Xy {
                x: image_wh.width * self.anchor_percent_xy.x,
                y: image_wh.height * self.anchor_percent_xy.y,
            })
        })
    }
    pub fn try_render_bounding_box(
        &self,
        time: Time,
        border_color: Color,
        border_width: Px,
        fill_color: Color,
    ) -> Option<RenderingTree> {
        let iamge_renderable = self.image_keyframe_graph.get_value(time)?;
        match iamge_renderable {
            ImageRenderable::Matrix { matrix, .. } => {
                let x = matrix.index_0_2().px();
                let y = matrix.index_1_2().px();

                let wh = self.get_image_px_wh(time)?;
                let anchor_xy = self.get_anchor_px_wh(time)?;

                let rotation_angle = Angle::Radian(matrix.index_1_0().atan2(matrix.index_0_0()));

                Some(translate(
                    x,
                    y,
                    rotate(
                        rotation_angle,
                        translate(
                            -anchor_xy.x,
                            -anchor_xy.y,
                            rect(RectParam {
                                rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
                                style: RectStyle {
                                    stroke: Some(RectStroke {
                                        color: border_color,
                                        width: border_width,
                                        border_position: BorderPosition::Inside,
                                    }),
                                    fill: Some(RectFill { color: fill_color }),
                                    round: None,
                                },
                            }),
                        ),
                    ),
                ))
            }
            ImageRenderable::Shader { .. } => {
                // TODO: Support render bounding box
                None
            }
        }
    }
    pub fn render_moving_path(&self) -> RenderingTree {
        let time_and_xys: Vec<(Time, Xy<Px>)> = self
            .image_keyframe_graph
            .get_point_line_tuples()
            .map(|(point, _line)| {
                let xy = Xy::new(point.value.x(), point.value.y());
                (point.time, xy)
            })
            .collect();

        let mut path_builder = PathBuilder::new();
        let paint_builder = PaintBuilder::new()
            .set_anti_alias(true)
            .set_style(PaintStyle::Stroke)
            .set_color(Color::grayscale_f01(0.0))
            .set_stroke_width(px(1.0));

        if let Some((_, xy)) = time_and_xys.first() {
            path_builder = path_builder.move_to(xy.x.into(), xy.y.into());
        }

        for (_, xy) in time_and_xys.iter().skip(1) {
            path_builder = path_builder.line_to(xy.x.into(), xy.y.into());
        }

        path(path_builder, paint_builder)
    }
}

impl Animate for AnimatableImage {
    fn render(&self, time: Time) -> RenderingTree {
        try_render(|| {
            let image_renderable = self.image_keyframe_graph.get_value(time)?;
            match image_renderable {
                ImageRenderable::Matrix { matrix, opacity } => {
                    if opacity <= OneZero::from(0.0) {
                        return None;
                    }
                    let source_url = self.image_source_url.as_ref()?.clone();

                    let image = crate::system::image::try_load(&source_url)?;
                    let image_size = image.size();

                    let anchor_xy = Xy {
                        x: image_size.width * self.anchor_percent_xy.x,
                        y: image_size.height * self.anchor_percent_xy.y,
                    };

                    let image_rendering_tree = namui::image(ImageParam {
                        rect: Rect::from_xy_wh(Xy::single(0.px()), image_size),
                        style: ImageStyle {
                            fit: ImageFit::Fill,
                            paint_builder: None,
                        },
                        source: ImageSource::Image(image),
                    });
                    let transformed_image = namui::transform(
                        matrix,
                        namui::translate(-anchor_xy.x, -anchor_xy.y, image_rendering_tree),
                    );

                    Some(transformed_image)
                }
                ImageRenderable::Shader { .. } => todo!(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;
    use wasm_bindgen_test::wasm_bindgen_test;

    struct LinearKeyframeLine {}

    impl KeyframeValue<LinearKeyframeLine, OneZero> for OneZero {
        fn interpolate(
            &self,
            next: &Self,
            context: &InterpolationContext<LinearKeyframeLine>,
        ) -> Self {
            let one_zero = OneZero::from(
                self.as_f32() * (1.0 - context.time_ratio) + next.as_f32() * context.time_ratio,
            );
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
