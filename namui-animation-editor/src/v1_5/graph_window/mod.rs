use namui::{
    animation::{KeyframeGraph, Layer},
    prelude::*,
    types::{PixelSize, Time, TimePerPixel},
};
use namui_prebuilt::{
    table::{fixed_closure, ratio_closure, vertical},
    *,
};
use std::sync::{Arc, RwLock};
mod render_graph;
use super::read_only_lock::ReadOnlyLock;
use render_graph::*;
mod time_ruler;
mod update;

pub(crate) struct GraphWindow {
    id: String,
    context: GraphWindowContext,
    x_context: PropertyContext<PixelSize>,
    y_context: PropertyContext<PixelSize>,
    width_context: PropertyContext<PixelSize>,
    height_context: PropertyContext<PixelSize>,
    mouse_over_row: Option<MouseOverRow>,
    row_height: Option<f32>,
    animation: ReadOnlyLock<animation::Animation>,
    selected_point_address: Option<PointAddress>,
    dragging: Option<Dragging>,
    playback_time: Time,
}

pub(crate) struct Props<'a> {
    pub layer: Option<&'a namui::animation::Layer>,
}

#[derive(Debug, Clone)]
enum Dragging {
    Point(PointAddress),
    Background {
        property_name: PropertyName,
        last_mouse_local_xy: Xy<f32>,
    },
}

#[derive(Debug, Clone)]
struct MouseOverRow {
    property_name: PropertyName,
    mouse_local_xy: Xy<f32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PointAddress {
    layer_id: String,
    property_name: PropertyName,
    point_id: String,
}

#[derive(Debug, Clone)]
enum Event {
    GraphMouseMoveIn {
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    },
    GraphMouseMoveOut,
    GraphShiftMouseWheel {
        delta: PixelSize,
    },
    GraphAltMouseWheel {
        delta: PixelSize,
        mouse_local_xy: Xy<f32>,
    },
    GraphCtrlMouseWheel {
        delta: PixelSize,
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    },
    GraphMouseRightDown {
        layer_id: String,
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    },
    GraphMouseLeftDown {
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
    },
    RowHeightChange {
        row_height: f32,
    },
    TimelineTimeRulerClicked {
        click_position_in_time: Time,
    },
    GraphPointMouseDown {
        point_address: PointAddress,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub(crate) fn new(animation: ReadOnlyLock<animation::Animation>) -> Self {
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
            },
            y_context: PropertyContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 10.0.into(),
                    pixel_size: 1.0.into(),
                },
            },
            width_context: PropertyContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 10.0.into(),
                    pixel_size: 1.0.into(),
                },
            },
            height_context: PropertyContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 10.0.into(),
                    pixel_size: 1.0.into(),
                },
            },
            mouse_over_row: None,
            row_height: None,
            animation,
            selected_point_address: None,
            dragging: None,
            playback_time: Time::zero(),
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

        render([
            vertical([
                fixed_closure(20.0, |wh| {
                    time_ruler::render(&time_ruler::Props {
                        start_at: self.context.start_at,
                        time_per_pixel: self.context.time_per_pixel,
                        xywh: XywhRect {
                            x: 0.0.into(),
                            y: 0.0.into(),
                            width: wh.width,
                            height: wh.height,
                        },
                    })
                }),
                ratio_closure(1.0, |wh| {
                    vertical([
                        ratio_closure(1.0, |wh| {
                            if self.row_height != Some(wh.height) {
                                namui::event::send(Event::RowHeightChange {
                                    row_height: wh.height,
                                });
                            }
                            self.render_pixel_size_graph_row(
                                wh,
                                layer,
                                PropertyName::X,
                                &layer.image.x,
                                &self.x_context,
                            )
                        }),
                        ratio!(1.0, |wh| {
                            self.render_pixel_size_graph_row(
                                wh,
                                layer,
                                PropertyName::Y,
                                &layer.image.y,
                                &self.y_context,
                            )
                        }),
                        ratio!(1.0, |wh| {
                            self.render_pixel_size_graph_row(
                                wh,
                                layer,
                                PropertyName::Width,
                                &layer.image.width,
                                &self.width_context,
                            )
                        }),
                        ratio!(1.0, |wh| {
                            self.render_pixel_size_graph_row(
                                wh,
                                layer,
                                PropertyName::Height,
                                &layer.image.height,
                                &self.height_context,
                            )
                        }),
                        ratio!(1.0, |wh| {
                            simple_rect(wh, Color::WHITE, 1.0, Color::BLACK)
                        }),
                        ratio!(1.0, |wh| {
                            simple_rect(wh, Color::WHITE, 1.0, Color::BLACK)
                        }),
                    ])(wh)
                    .attach_event(|builder| {
                        builder.on_mouse_move_out(|_| {
                            namui::event::send(Event::GraphMouseMoveOut);
                        })
                    })
                }),
            ])(wh),
            render_playback_time_line(
                wh,
                self.playback_time,
                self.context.start_at,
                self.context.time_per_pixel,
            ),
        ])
    }
}

impl GraphWindow {
    fn render_pixel_size_graph_row(
        &self,
        wh: Wh<f32>,
        layer: &Layer,
        property_name: PropertyName,
        graph: &KeyframeGraph<PixelSize>,
        property_context: &PropertyContext<PixelSize>,
    ) -> RenderingTree {
        render_graph_row(
            wh,
            layer,
            property_name,
            (
                graph,
                Context {
                    start_at: self.context.start_at,
                    time_per_pixel: self.context.time_per_pixel,
                    value_at_bottom: property_context.value_at_bottom,
                    value_per_pixel: property_context.value_per_pixel,
                    mouse_local_xy: self.mouse_over_row.as_ref().and_then(|mouse_over_row| {
                        if mouse_over_row.property_name == property_name {
                            Some(mouse_over_row.mouse_local_xy)
                        } else {
                            None
                        }
                    }),
                    property_name,
                    selected_point_id: self.selected_point_address.as_ref().and_then(
                        |selected_point_address| {
                            if selected_point_address.property_name == property_name {
                                Some(selected_point_address.point_id.clone())
                            } else {
                                None
                            }
                        },
                    ),
                    layer,
                },
            ),
        )
    }
}

fn render_playback_time_line(
    wh: Wh<f32>,
    playback_time: Time,
    start_at: Time,
    time_per_pixel: TimePerPixel,
) -> RenderingTree {
    let x = (playback_time - start_at) / time_per_pixel;
    let color = Color::RED;
    let path = namui::PathBuilder::new()
        .move_to(x.into(), 0.0)
        .line_to(x.into(), wh.height);
    let paint = namui::PaintBuilder::new()
        .set_color(color)
        .set_style(namui::PaintStyle::Stroke)
        .set_stroke_width(1.0);

    namui::path(path, paint)
}

fn render_graph_row(
    wh: Wh<f32>,
    layer: &Layer,
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
        let layer_id = layer.id.clone();
        builder
            .on_mouse_move_in(move |event| {
                namui::event::send(Event::GraphMouseMoveIn {
                    property_name,
                    mouse_local_xy: event.local_xy,
                    row_wh: wh,
                })
            })
            .on_mouse_down(move |event| match event.button {
                Some(MouseButton::Left) => namui::event::send(Event::GraphMouseLeftDown {
                    property_name,
                    mouse_local_xy: event.local_xy,
                }),
                Some(MouseButton::Right) => namui::event::send(Event::GraphMouseRightDown {
                    layer_id: layer_id.clone(),
                    property_name,
                    mouse_local_xy: event.local_xy,
                    row_wh: wh,
                }),
                _ => {}
            })
            .on_wheel(move |event| {
                let managers = namui::managers();
                let mouse_global_xy = managers.mouse_manager.mouse_position();
                let row_xy = event
                    .namui_context
                    .get_rendering_tree_xy(event.target)
                    .expect("ERROR: fail to get rendering_tree_xy");

                let mouse_local_xy = Xy {
                    x: mouse_global_xy.x as f32 - row_xy.x,
                    y: mouse_global_xy.y as f32 - row_xy.y,
                };

                if mouse_local_xy.x < 0.0
                    || wh.width < mouse_local_xy.x
                    || mouse_local_xy.y < 0.0
                    || wh.height < mouse_local_xy.y
                {
                    return;
                }

                if managers
                    .keyboard_manager
                    .any_code_press([namui::Code::ShiftLeft, namui::Code::ShiftRight])
                {
                    namui::event::send(Event::GraphShiftMouseWheel {
                        delta: PixelSize(event.delta_xy.y),
                    })
                } else if managers
                    .keyboard_manager
                    .any_code_press([namui::Code::AltLeft, namui::Code::AltRight])
                {
                    namui::event::send(Event::GraphAltMouseWheel {
                        delta: PixelSize(event.delta_xy.y),
                        mouse_local_xy,
                    })
                } else if managers
                    .keyboard_manager
                    .any_code_press([namui::Code::ControlLeft, namui::Code::ControlRight])
                {
                    namui::event::send(Event::GraphCtrlMouseWheel {
                        delta: PixelSize(event.delta_xy.y),
                        mouse_local_xy,
                        property_name,
                        row_wh: wh,
                    })
                }
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

struct Context<'a, TValue> {
    start_at: Time,
    time_per_pixel: TimePerPixel,
    value_per_pixel: ValuePerPixel<TValue>,
    value_at_bottom: TValue,
    mouse_local_xy: Option<Xy<f32>>,
    property_name: PropertyName,
    selected_point_id: Option<String>,
    layer: &'a Layer,
}

#[derive(Debug, Clone, Copy)]
struct PropertyContext<TValue> {
    value_per_pixel: ValuePerPixel<TValue>,
    value_at_bottom: TValue,
}
