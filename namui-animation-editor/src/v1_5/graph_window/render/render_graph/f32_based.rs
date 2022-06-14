use super::*;
use namui::animation::KeyframePoint;

impl<TValue: KeyframeValue + Copy + From<f32> + Into<f32>> RenderGraph
    for (&'_ KeyframeGraph<TValue>, Context<'_, TValue>)
{
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        let x_axis_guide_lines = self.render_x_axis_guide_lines(wh);
        let mouse_guide = self.render_mouse_guide(wh);
        let point_and_lines = self.render_point_and_lines(wh);

        namui::clip(
            namui::PathBuilder::new().add_rect(&LtrbRect {
                left: 0.0,
                top: 0.0,
                right: wh.width,
                bottom: wh.height,
            }),
            ClipOp::Intersect,
            render([x_axis_guide_lines, mouse_guide, point_and_lines]),
        )
    }

    fn render_x_axis_guide_lines(&self, wh: Wh<f32>) -> RenderingTree {
        let (_, context) = self;
        const BOLD_GRADATION_INTERVAL: usize = 2;

        let value_at_top = context.value_at_bottom.into()
            + (context.value_per_pixel * PixelSize(wh.height)).into();

        let gradation_interval = {
            let gradation_value_candidates: Vec<_> = [5, 10, 25, 50, 100, 200, 500]
                .into_iter()
                .map(|value| TValue::from(value as f32))
                .collect();

            let last = *gradation_value_candidates.last().unwrap();

            gradation_value_candidates
                .into_iter()
                .find(|value| {
                    let px = context.value_per_pixel.get_pixel_size(*value);
                    PixelSize(10.0) <= px && px <= PixelSize(50.0)
                })
                .unwrap_or(last)
        };

        enum Gradation {
            Bold { y: PixelSize, value: PixelSize },
            Light { y: PixelSize },
        }

        let gradations = {
            let mut gradations = vec![];

            let mut value = 0.0;
            let mut index = 0;
            while value > context.value_at_bottom.into() {
                value -= gradation_interval.into();
            }
            while value < context.value_at_bottom.into() {
                value += gradation_interval.into();
                index += 1;
            }

            while value < value_at_top {
                let y = PixelSize(wh.height)
                    - context
                        .value_per_pixel
                        .get_pixel_size((value - context.value_at_bottom.into()).into());

                match index % BOLD_GRADATION_INTERVAL {
                    0 => gradations.push(Gradation::Bold {
                        y,
                        value: value.into(),
                    }),
                    _ => gradations.push(Gradation::Light { y }),
                }
                value += gradation_interval.into();
                index += 1;
            }
            gradations
        };

        fn bold_line(wh: Wh<f32>, y: PixelSize, value: PixelSize) -> RenderingTree {
            let path_builder = namui::PathBuilder::new()
                .move_to(0.0, 0.0)
                .line_to(wh.width, 0.0);
            let painter_builder = namui::PaintBuilder::new()
                .set_stroke_width(1.0)
                .set_style(namui::PaintStyle::Stroke)
                .set_color(namui::Color::from_u8(0, 128, 0, 255));

            let gradation_label = namui::text(namui::TextParam {
                x: 0.0,
                y: 0.0,
                align: TextAlign::Left,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::LIGHT,
                    language: Language::Ko,
                    serif: false,
                    size: 10,
                },
                style: TextStyle {
                    background: Some(TextStyleBackground {
                        color: Color::BLACK,
                        ..Default::default()
                    }),
                    color: Color::WHITE,
                    ..Default::default()
                },
                text: format!("{}", f32::from(value)),
            });

            namui::translate(
                0.0,
                y.into(),
                render([namui::path(path_builder, painter_builder), gradation_label]),
            )
        }

        fn light_line(wh: Wh<f32>, y: PixelSize) -> RenderingTree {
            let path_builder = namui::PathBuilder::new()
                .move_to(0.0, 0.0)
                .line_to(wh.width, 0.0);
            let painter_builder = namui::PaintBuilder::new()
                .set_stroke_width(1.0)
                .set_style(namui::PaintStyle::Stroke)
                .set_color(namui::Color::from_u8(0, 64, 0, 255));

            namui::translate(
                0.0,
                y.into(),
                render([namui::path(path_builder, painter_builder)]),
            )
        }

        render(gradations.iter().map(|gradation| match gradation {
            Gradation::Bold { y, value } => bold_line(wh, *y, *value),
            Gradation::Light { y } => light_line(wh, *y),
        }))
    }

    fn render_mouse_guide(&self, wh: Wh<f32>) -> RenderingTree {
        let (_, context) = self;
        let mouse_local_xy = {
            match context.mouse_local_xy {
                Some(mouse_local_xy) => mouse_local_xy,
                None => return RenderingTree::Empty,
            }
        };

        let time_at_x = context.start_at + context.time_per_pixel * PixelSize(mouse_local_xy.x);

        let value_at_y = context.value_at_bottom.into()
            + (context.value_per_pixel * PixelSize(wh.height - mouse_local_xy.y)).into();

        let label = namui::text(namui::TextParam {
            x: 7.0,
            y: -3.0,
            align: TextAlign::Left,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                font_weight: FontWeight::LIGHT,
                language: Language::Ko,
                serif: false,
                size: 10,
            },
            style: TextStyle {
                background: Some(TextStyleBackground {
                    color: Color::BLACK,
                    ..Default::default()
                }),
                color: Color::WHITE,
                ..Default::default()
            },
            text: format!(
                "{:.1}s / {:.0}px",
                time_at_x.get_total_milliseconds() / 1000.0,
                Into::<f32>::into(value_at_y)
            ),
        });

        namui::translate(mouse_local_xy.x, mouse_local_xy.y, label)
    }

    fn render_point_and_lines(&self, wh: Wh<f32>) -> RenderingTree {
        let (graph, context) = self;

        let mut iter = graph.get_points_with_lines().iter().peekable();
        let mut rendered = vec![];

        while let Some((point, _)) = iter.next() {
            let next_point_line = iter.peek();

            let point_xy = get_xy_of_point(wh, context, point);
            let point_address = PointAddress {
                layer_id: context.layer.id.clone(),
                property_name: context.property_name,
                point_id: point.id().to_string(),
            };
            rendered.push(render_point_xy(
                point_xy,
                context.mouse_local_xy,
                point_address,
                context.selected_point_id == Some(point.id().to_string()),
            ));

            if let Some((next_point, _)) = next_point_line {
                let next_point_xy = get_xy_of_point(wh, context, next_point);
                rendered.push(render_line(point_xy, next_point_xy));
            }
        }

        render(rendered)
    }
}

fn render_point_xy(
    xy: Xy<PixelSize>,
    mouse_local_xy: Option<Xy<f32>>,
    point_address: PointAddress,
    is_selected: bool,
) -> RenderingTree {
    const RADIUS: f32 = 4.0;

    let is_mouse_on_point = match mouse_local_xy {
        Some(mouse_local_xy) => {
            (mouse_local_xy.x - f32::from(xy.x)).abs() < RADIUS
                && (mouse_local_xy.y - f32::from(xy.y)).abs() < RADIUS
        }
        None => false,
    };

    let point_builder = namui::PathBuilder::new()
        .add_oval(&LtrbRect {
            left: -RADIUS,
            top: -RADIUS,
            right: RADIUS,
            bottom: RADIUS,
        })
        .close();

    let color = if is_mouse_on_point || is_selected {
        Color::from_u8(255, 255, 0, 255)
    } else {
        Color::from_u8(255, 0, 0, 255)
    };
    let painter_builder = namui::PaintBuilder::new()
        .set_style(namui::PaintStyle::Fill)
        .set_color(color);

    namui::translate(
        xy.x.into(),
        xy.y.into(),
        namui::path(point_builder, painter_builder),
    )
    .attach_event(|builder| {
        let point_address = point_address.clone();
        builder.on_mouse_down(move |_| {
            namui::event::send(Event::GraphPointMouseDown {
                point_address: point_address.clone(),
            })
        })
    })
}

fn render_line(from: Xy<PixelSize>, to: Xy<PixelSize>) -> RenderingTree {
    let path_builder = namui::PathBuilder::new()
        .move_to(from.x.into(), from.y.into())
        .line_to(to.x.into(), to.y.into());
    let painter_builder = namui::PaintBuilder::new()
        .set_stroke_width(1.0)
        .set_style(namui::PaintStyle::Stroke)
        .set_color(namui::Color::from_u8(128, 0, 0, 255));

    namui::path(path_builder, painter_builder)
}

fn get_xy_of_point<TValue: KeyframeValue + Copy + From<f32> + Into<f32>>(
    wh: Wh<f32>,
    context: &Context<TValue>,
    point: &KeyframePoint<TValue>,
) -> Xy<PixelSize> {
    let x = (point.time - context.start_at) / context.time_per_pixel;
    let y = PixelSize(wh.height)
        - context
            .value_per_pixel
            .get_pixel_size((point.value.into() - context.value_at_bottom.into()).into());
    Xy { x, y }
}
