use namui::animation::KeyframePoint;

use super::*;

impl RenderGraph for (&'_ KeyframeGraph<PixelSize>, Context<PixelSize>) {
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        let x_axis_guide_lines = self.render_x_axis_guide_lines(wh);
        let mouse_guide = self.render_mouse_guide(wh);
        let points = self.render_point_and_lines(wh);

        render([x_axis_guide_lines, mouse_guide, points])
    }

    fn render_x_axis_guide_lines(&self, wh: Wh<f32>) -> RenderingTree {
        let (_, context) = self;
        const BOLD_GRADATION_INTERVAL: usize = 2;

        let value_at_top = context.value_at_bottom + context.value_per_pixel * PixelSize(wh.height);

        let gradation_interval = {
            let gradation_value_candidates: Vec<_> = [5, 10, 25, 50, 100, 200, 500]
                .into_iter()
                .map(|value| PixelSize(value as f32))
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

            let mut value = PixelSize(0.0);
            let mut index = 0;
            while value > context.value_at_bottom {
                value -= gradation_interval;
            }
            while value < context.value_at_bottom {
                value += gradation_interval;
                index += 1;
            }

            while value < value_at_top {
                let y = PixelSize(wh.height)
                    - context
                        .value_per_pixel
                        .get_pixel_size(value - context.value_at_bottom);

                match index % BOLD_GRADATION_INTERVAL {
                    0 => gradations.push(Gradation::Bold { y, value }),
                    _ => gradations.push(Gradation::Light { y }),
                }
                value += gradation_interval;
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

        let value_at_y = context.value_at_bottom
            + context.value_per_pixel * PixelSize(wh.height - mouse_local_xy.y);

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
                value_at_y.0
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
            rendered.push(render_point_xy(point_xy));

            if let Some((next_point, _)) = next_point_line {
                let next_point_xy = get_xy_of_point(wh, context, next_point);
                rendered.push(render_line(point_xy, next_point_xy));
            }
        }

        render(rendered)
    }
}

fn render_point_xy(xy: Xy<PixelSize>) -> RenderingTree {
    const RADIUS: f32 = 2.0;
    let point_builder = namui::PathBuilder::new()
        .add_oval(&LtrbRect {
            left: -RADIUS,
            top: -RADIUS,
            right: RADIUS,
            bottom: RADIUS,
        })
        .close();
    let painter_builder = namui::PaintBuilder::new()
        .set_style(namui::PaintStyle::Fill)
        .set_color(namui::Color::RED);

    namui::translate(
        xy.x.into(),
        xy.y.into(),
        namui::path(point_builder, painter_builder),
    )
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

fn get_xy_of_point(
    wh: Wh<f32>,
    context: &Context<PixelSize>,
    point: &KeyframePoint<PixelSize>,
) -> Xy<PixelSize> {
    let x = (point.time - context.start_at) / context.time_per_pixel;
    let y = PixelSize(wh.height)
        - context
            .value_per_pixel
            .get_pixel_size(point.value - context.value_at_bottom);
    Xy { x, y }
}
