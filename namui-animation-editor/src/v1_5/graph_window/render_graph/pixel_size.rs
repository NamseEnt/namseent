use super::*;

impl RenderGraph for (&'_ KeyframeGraph<PixelSize>, Context<PixelSize>) {
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        let (graph, context) = self;

        let x_axis_guide_lines = self.render_x_axis_guide_lines(wh);
        let mouse_guide = self.render_mouse_guide(wh);

        render([x_axis_guide_lines, mouse_guide])
    }

    fn render_x_axis_guide_lines(&self, wh: Wh<f32>) -> RenderingTree {
        let (_, context) = self;

        let value_at_top = context.value_at_bottom + context.value_per_pixel * PixelSize(wh.height);

        let gradation_interval = {
            let gradation_value_candidates: Vec<_> = [5, 10, 20, 50, 100, 200, 500]
                .into_iter()
                .map(|value| PixelSize(value as f32))
                .collect();

            let last = *gradation_value_candidates.last().unwrap();

            gradation_value_candidates
                .into_iter()
                .find(|value| {
                    let px = context.value_per_pixel.get_pixel_size(*value);
                    PixelSize(10.0) <= px && px <= PixelSize(40.0)
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
            while value < context.value_at_bottom {
                value += gradation_interval;
                index += 1;
            }

            while value < value_at_top {
                let y = PixelSize(wh.height) - context.value_per_pixel.get_pixel_size(value);

                match index % 5 {
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
}
