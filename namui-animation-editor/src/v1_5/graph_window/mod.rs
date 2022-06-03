use namui::{
    animation::{KeyframeGraph, KeyframePoint, KeyframeValue, Layer},
    prelude::*,
    types::{Angle, PixelSize, Time, TimePerPixel},
};
use namui_prebuilt::{
    table::{horizontal, ratio_closure, vertical},
    typography::center_text,
    *,
};
use std::sync::{Arc, RwLock};

pub(crate) struct GraphWindow {
    id: String,
    context: GraphWindowContext,
    x_context: YAxisContext<PixelSize>,
}

pub(crate) struct Props<'a> {
    pub layer: Option<&'a namui::animation::Layer>,
}

enum Event {}

pub(crate) struct GraphWindowContext {
    pub start_at: Time,
    pub time_per_pixel: TimePerPixel,
}

impl GraphWindow {
    pub(crate) fn new() -> Self {
        Self {
            id: namui::nanoid(),
            context: GraphWindowContext {
                start_at: Time::zero(),
                time_per_pixel: Time::from_ms(50.0) / PixelSize::new(1.0),
            },
            x_context: YAxisContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 10.0.into(),
                    pixel_size: 1.0.into(),
                },
            },
        }
    }
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {}

    fn get_row_column_count(&self, wh: Wh<f32>, props: Props) -> (usize, usize) {
        (8, 8)
    }
}

impl table::CellRender<Props<'_>> for GraphWindow {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        if props.layer.is_none() {
            return simple_rect(wh, Color::WHITE, 1.0, Color::BLACK);
        }
        let layer = props.layer.unwrap();

        // x: KeyframeGraph<PixelSize>,
        // y: KeyframeGraph<PixelSize>,
        // width: KeyframeGraph<PixelSize>,
        // height: KeyframeGraph<PixelSize>,
        // rotation_angle: KeyframeGraph<Angle>,
        // opacity: KeyframeGraph<OneZero>,

        vertical([
            ratio_closure(1.0, |wh| {
                render_graph_row(
                    wh,
                    &self.context,
                    "X",
                    (
                        &layer.image.x,
                        Context {
                            start_at: self.context.start_at,
                            time_per_pixel: self.context.time_per_pixel,
                            value_at_bottom: self.x_context.value_at_bottom,
                            value_per_pixel: self.x_context.value_per_pixel,
                        },
                    ),
                )
            }),
            ratio!(1.0, |wh| {
                simple_rect(wh, Color::WHITE, 1.0, Color::BLACK)
            }),
            ratio!(1.0, |wh| {
                simple_rect(wh, Color::WHITE, 1.0, Color::BLACK)
            }),
            ratio!(1.0, |wh| {
                simple_rect(wh, Color::WHITE, 1.0, Color::BLACK)
            }),
            ratio!(1.0, |wh| {
                simple_rect(wh, Color::WHITE, 1.0, Color::BLACK)
            }),
            ratio!(1.0, |wh| {
                simple_rect(wh, Color::WHITE, 1.0, Color::BLACK)
            }),
        ])(wh)
    }
}

trait RenderGraph {
    fn render(&self, wh: Wh<f32>) -> RenderingTree;
    fn draw_x_axis_guide_lines(&self, wh: Wh<f32>) -> RenderingTree;
}

fn render_graph_row(
    wh: Wh<f32>,
    context: &GraphWindowContext,
    name: impl AsRef<str>,
    render_graph: impl RenderGraph,
) -> RenderingTree {
    let label_wh = Wh {
        width: 30.0,
        height: wh.height / 8.0,
    };
    let label = render([
        simple_rect(label_wh, Color::BLACK, 1.0, Color::WHITE),
        namui_prebuilt::typography::body::center(label_wh, name, Color::BLACK),
    ]);
    render([
        simple_rect(wh, Color::WHITE, 1.0, Color::BLACK),
        render_graph.render(wh),
        label,
    ])
}

#[derive(Debug, Clone, Copy)]
struct ValuePerPixel<TValue> {
    value: TValue,
    pixel_size: PixelSize,
}

impl<TValue: std::ops::Mul<f32, Output = TValue>> std::ops::Mul<PixelSize>
    for ValuePerPixel<TValue>
{
    type Output = TValue;

    fn mul(self, rhs: PixelSize) -> Self::Output {
        self.value * (rhs / self.pixel_size)
    }
}
impl<TValue: std::ops::Div<Output = f32> + Copy> ValuePerPixel<TValue> {
    fn get_pixel_size(&self, value: TValue) -> PixelSize {
        (value / self.value) * self.pixel_size
    }
}

struct Context<TValue> {
    start_at: Time,
    time_per_pixel: TimePerPixel,
    value_per_pixel: ValuePerPixel<TValue>,
    value_at_bottom: TValue,
}

#[derive(Debug, Clone, Copy)]
struct YAxisContext<TValue> {
    value_per_pixel: ValuePerPixel<TValue>,
    value_at_bottom: TValue,
}

impl RenderGraph for (&'_ KeyframeGraph<PixelSize>, Context<PixelSize>) {
    fn render(&self, wh: Wh<f32>) -> RenderingTree {
        let (graph, context) = self;

        let x_axis_guide_lines = self.draw_x_axis_guide_lines(wh);

        render([x_axis_guide_lines])
    }

    fn draw_x_axis_guide_lines(&self, wh: Wh<f32>) -> RenderingTree {
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

        let (bold_line_ys, light_line_ys) = {
            let mut bold_line_ys = vec![];
            let mut light_line_ys = vec![];

            let mut value = PixelSize(0.0);
            let mut index = 0;
            while value < context.value_at_bottom {
                value += gradation_interval;
                index += 1;
            }

            while value < value_at_top {
                let y = PixelSize(wh.height) - context.value_per_pixel.get_pixel_size(value);

                match index % 5 {
                    0 => bold_line_ys.push(y),
                    _ => light_line_ys.push(y),
                }
                value += gradation_interval;
                index += 1;
            }
            (bold_line_ys, light_line_ys)
        };

        let path_builder = namui::PathBuilder::new()
            .move_to(0.0, 0.0)
            .line_to(wh.width, 0.0);

        let bold_line = {
            let painter_builder = namui::PaintBuilder::new()
                .set_stroke_width(2.0)
                .set_style(namui::PaintStyle::Stroke)
                .set_color(namui::Color::from_u8(0, 0, 255, 255));

            namui::path(path_builder.clone(), painter_builder)
        };

        let light_line = {
            let painter_builder = namui::PaintBuilder::new()
                .set_stroke_width(1.0)
                .set_style(namui::PaintStyle::Stroke)
                .set_color(namui::Color::from_u8(0, 0, 255, 255));

            namui::path(path_builder, painter_builder)
        };

        render(
            bold_line_ys
                .iter()
                .map(|y| namui::translate(0.0, y.into(), bold_line.clone()))
                .chain(
                    light_line_ys
                        .iter()
                        .map(|y| namui::translate(0.0, y.into(), light_line.clone())),
                ),
        )
    }
}
