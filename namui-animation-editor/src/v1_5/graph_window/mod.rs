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
    mouse_over_row: Option<MouseOverRow>,
    row_height: Option<f32>,
}

pub(crate) struct Props<'a> {
    pub layer: Option<&'a namui::animation::Layer>,
}

struct MouseOverRow {
    property_name: PropertyName,
    local_xy: Xy<f32>,
}

enum Event {
    GraphMouseMoveIn {
        property_name: PropertyName,
        local_xy: Xy<f32>,
    },
    GraphMouseMoveOut {
        property_name: PropertyName,
    },
    GraphShiftMouseWheel {
        delta: PixelSize,
    },
    GraphAltMouseWheel {
        delta: PixelSize,
        anchor_xy: Xy<f32>,
    },
    GraphCtrlMouseWheel {
        delta: PixelSize,
        property_name: PropertyName,
        anchor_xy: Xy<f32>,
        row_wh: Wh<f32>,
    },
    RowHeightChange {
        row_height: f32,
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
            mouse_over_row: None,
            row_height: None,
        }
    }
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::GraphMouseMoveIn {
                    property_name,
                    local_xy,
                } => {
                    self.mouse_over_row = Some(MouseOverRow {
                        property_name: *property_name,
                        local_xy: *local_xy,
                    });
                    match property_name {
                        PropertyName::X => {
                            self.x_context.mouse_local_xy = Some(*local_xy);
                        }
                        PropertyName::Y => todo!(),
                        PropertyName::Width => todo!(),
                        PropertyName::Height => todo!(),
                    }
                }
                Event::GraphMouseMoveOut { property_name } => {
                    if self
                        .mouse_over_row
                        .as_ref()
                        .map(|row| row.property_name == *property_name)
                        == Some(true)
                    {
                        self.mouse_over_row = None;
                    }
                    match property_name {
                        PropertyName::X => {
                            self.x_context.mouse_local_xy = None;
                        }
                        PropertyName::Y => todo!(),
                        PropertyName::Width => todo!(),
                        PropertyName::Height => todo!(),
                    }
                }
                Event::GraphShiftMouseWheel { delta } => {
                    self.context.start_at += delta * self.context.time_per_pixel;
                }
                Event::GraphAltMouseWheel { delta, anchor_xy } => {
                    let time_at_mouse_position = self.context.start_at
                        + PixelSize(anchor_xy.x) * self.context.time_per_pixel;

                    let next_time_per_pixel =
                        zoom_time_per_pixel(self.context.time_per_pixel, delta.into());

                    let next_start_at =
                        time_at_mouse_position - PixelSize(anchor_xy.x) * next_time_per_pixel;

                    self.context.time_per_pixel = next_time_per_pixel;
                    self.context.start_at = next_start_at;
                }
                Event::GraphCtrlMouseWheel {
                    delta,
                    property_name,
                    anchor_xy,
                    row_wh,
                } => match property_name {
                    PropertyName::X => {
                        let value_at_mouse_position = self.x_context.value_at_bottom
                            + self.x_context.value_per_pixel
                                * PixelSize(row_wh.height - anchor_xy.y);

                        let next_value_per_pixel =
                            zoom_pixel_size_per_pixel(self.x_context.value_per_pixel, delta.into());

                        let next_value_at_bottom = value_at_mouse_position
                            - next_value_per_pixel * PixelSize(row_wh.height - anchor_xy.y);

                        self.x_context.value_per_pixel = next_value_per_pixel;
                        self.x_context.value_at_bottom = next_value_at_bottom;
                    }
                    PropertyName::Y => todo!(),
                    PropertyName::Width => todo!(),
                    PropertyName::Height => todo!(),
                },
                Event::RowHeightChange { row_height } => {
                    self.row_height = Some(*row_height);
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            if let NamuiEvent::KeyDown(event) = event {
                self.handle_key_down(event);
            }
        }
    }

    fn handle_key_down(&mut self, event: &KeyEvent) {
        if self.row_height.is_none() {
            return;
        }
        let row_height = self.row_height.unwrap();
        if self.mouse_over_row.is_none() {
            return;
        }
        let mouse_over_row = self.mouse_over_row.as_ref().unwrap();

        enum Arrow {
            Left,
            Right,
            Top,
            Bottom,
        }
        let arrow = match event.code {
            Code::ArrowLeft => Arrow::Left,
            Code::ArrowRight => Arrow::Right,
            Code::ArrowUp => Arrow::Top,
            Code::ArrowDown => Arrow::Bottom,
            _ => return,
        };

        let managers = namui::managers();
        if managers
            .keyboard_manager
            .any_code_press([Code::AltLeft, Code::AltRight])
        {
            match arrow {
                Arrow::Left | Arrow::Right => {
                    let time_at_mouse_position = self.context.start_at
                        + PixelSize(mouse_over_row.local_xy.x) * self.context.time_per_pixel;

                    let next_time_per_pixel = zoom_time_per_pixel(
                        self.context.time_per_pixel,
                        match arrow {
                            Arrow::Left => -10.0,
                            Arrow::Right => 10.0,
                            _ => unreachable!(),
                        },
                    );

                    let next_start_at = time_at_mouse_position
                        - PixelSize(mouse_over_row.local_xy.x) * next_time_per_pixel;

                    self.context.time_per_pixel = next_time_per_pixel;
                    self.context.start_at = next_start_at;
                }
                Arrow::Top | Arrow::Bottom => match mouse_over_row.property_name {
                    PropertyName::X => {
                        let value_at_mouse_position = self.x_context.value_at_bottom
                            + self.x_context.value_per_pixel
                                * PixelSize(row_height - mouse_over_row.local_xy.y);

                        let next_value_per_pixel = zoom_pixel_size_per_pixel(
                            self.x_context.value_per_pixel,
                            match arrow {
                                Arrow::Top => 10.0,
                                Arrow::Bottom => -10.0,
                                _ => unreachable!(),
                            },
                        );

                        let next_value_at_bottom = value_at_mouse_position
                            - next_value_per_pixel
                                * PixelSize(row_height - mouse_over_row.local_xy.y);

                        self.x_context.value_per_pixel = next_value_per_pixel;
                        self.x_context.value_at_bottom = next_value_at_bottom;
                    }
                    PropertyName::Y => todo!(),
                    PropertyName::Width => todo!(),
                    PropertyName::Height => todo!(),
                },
            }
        } else if managers
            .keyboard_manager
            .any_code_press([Code::ControlLeft, Code::ControlRight])
        {
        }
    }
    // else if todo!() && selected_point.is_some() {
    //     todo!()
    // }
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
                if self.row_height != Some(wh.height) {
                    namui::event::send(Event::RowHeightChange {
                        row_height: wh.height,
                    });
                }
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
            .on_mouse_move_in(move |event| {
                namui::event::send(Event::GraphMouseMoveIn {
                    property_name,
                    local_xy: event.local_xy,
                })
            })
            .on_mouse_move_out(move |_| {
                namui::event::send(Event::GraphMouseMoveOut { property_name })
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
                        anchor_xy: mouse_local_xy,
                    })
                } else if managers
                    .keyboard_manager
                    .any_code_press([namui::Code::ControlLeft, namui::Code::ControlRight])
                {
                    namui::event::send(Event::GraphCtrlMouseWheel {
                        delta: PixelSize(event.delta_xy.y),
                        anchor_xy: mouse_local_xy,
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

fn zoom_time_per_pixel(target: TimePerPixel, delta: f32) -> TimePerPixel {
    const STEP: f32 = 400.0;
    const MIN: f32 = 10.0;
    const MAX: f32 = 1000.0;

    let ms_per_pixel = target.ms_per_pixel();

    let wheel = STEP * (ms_per_pixel / 10.0).log2();

    let next_wheel = wheel + delta;

    let zoomed = namui::math::num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);
    TimePerPixel::from_ms_per_pixel(zoomed)
}

fn zoom_pixel_size_per_pixel(
    target: ValuePerPixel<PixelSize>,
    delta: f32,
) -> ValuePerPixel<PixelSize> {
    const STEP: f32 = 400.0;
    const MIN: f32 = 1.0;
    const MAX: f32 = 100.0;

    let wheel = STEP * (target.value / target.pixel_size / 10.0).log2();

    let next_wheel = wheel + delta;

    let zoomed = namui::math::num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);

    ValuePerPixel {
        value: zoomed.into(),
        pixel_size: 1.0_f32.into(),
    }
}
