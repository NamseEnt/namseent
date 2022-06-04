use namui::{
    animation::KeyframeGraph,
    prelude::*,
    types::{PixelSize, Time, TimePerPixel},
};
use namui_prebuilt::{
    table::{ratio_closure, vertical},
    *,
};

mod render_graph;
use render_graph::*;

pub(crate) struct GraphWindow {
    id: String,
    context: GraphWindowContext,
    x_context: PropertyContext<PixelSize>,
}

pub(crate) struct Props<'a> {
    pub layer: Option<&'a namui::animation::Layer>,
}

enum Event {
    GraphMouseMoveIn {
        property_name: PropertyName,
        local_xy: Xy<f32>,
    },
    GraphMouseMoveOut {
        property_name: PropertyName,
    },
}

enum PropertyName {
    X,
    Y,
    Width,
    Height,
}

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
            x_context: PropertyContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 10.0.into(),
                    pixel_size: 1.0.into(),
                },
                mouse_local_xy: None,
            },
        }
    }
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::GraphMouseMoveIn {
                    property_name,
                    local_xy,
                } => match property_name {
                    PropertyName::X => {
                        self.x_context.mouse_local_xy = Some(*local_xy);
                    }
                    PropertyName::Y => todo!(),
                    PropertyName::Width => todo!(),
                    PropertyName::Height => todo!(),
                },
                Event::GraphMouseMoveOut { property_name } => match property_name {
                    PropertyName::X => {
                        self.x_context.mouse_local_xy = None;
                    }
                    PropertyName::Y => todo!(),
                    PropertyName::Width => todo!(),
                    PropertyName::Height => todo!(),
                },
            }
        }
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
                    PropertyName::X,
                    (
                        &layer.image.x,
                        Context {
                            start_at: self.context.start_at,
                            time_per_pixel: self.context.time_per_pixel,
                            value_at_bottom: self.x_context.value_at_bottom,
                            value_per_pixel: self.x_context.value_per_pixel,
                            mouse_local_xy: self.x_context.mouse_local_xy,
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

fn render_graph_row(
    wh: Wh<f32>,
    context: &GraphWindowContext,
    property_name: PropertyName,
    render_graph: impl RenderGraph,
) -> RenderingTree {
    let label_wh = Wh {
        width: 30.0,
        height: wh.height / 8.0,
    };
    let label = render([
        simple_rect(label_wh, Color::BLACK, 1.0, Color::WHITE),
        namui_prebuilt::typography::body::center(
            label_wh,
            match property_name {
                PropertyName::X => "X",
                PropertyName::Y => "Y",
                PropertyName::Width => "Width",
                PropertyName::Height => "Height",
            },
            Color::BLACK,
        ),
    ]);
    render([
        simple_rect(wh, Color::WHITE, 1.0, Color::BLACK),
        render_graph.render(wh),
        label,
    ])
    .attach_event(|builder| {
        builder
            .on_mouse_move_in(|event| {
                namui::event::send(Event::GraphMouseMoveIn {
                    property_name: PropertyName::X,
                    local_xy: event.local_xy,
                })
            })
            .on_mouse_move_out(|_| {
                namui::event::send(Event::GraphMouseMoveOut {
                    property_name: PropertyName::X,
                })
            })
    })
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
    mouse_local_xy: Option<Xy<f32>>,
}

#[derive(Debug, Clone, Copy)]
struct PropertyContext<TValue> {
    value_per_pixel: ValuePerPixel<TValue>,
    value_at_bottom: TValue,
    mouse_local_xy: Option<Xy<f32>>,
}
