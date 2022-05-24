use super::*;
mod render_graph;
use namui::{animation::KeyframeValue, types::OneZero};
use render_graph::*;

impl table::CellRender<Props<'_>> for GraphWindow {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        if props.layer.is_none() {
            return simple_rect(wh, Color::WHITE, 1.0, Color::BLACK);
        }
        let layer = props.layer.unwrap();

        render([
            vertical([
                fixed(20.0, |wh| {
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
                ratio(1.0, |wh| {
                    vertical([
                        ratio(1.0, |wh| {
                            if self.row_height != Some(wh.height) {
                                namui::event::send(Event::RowHeightChange {
                                    row_height: wh.height,
                                });
                            }
                            self.render_f32_based_graph_row(
                                wh,
                                layer,
                                PropertyName::X,
                                &layer.image.x,
                                &self.x_context,
                            )
                        }),
                        ratio(1.0, |wh| {
                            self.render_f32_based_graph_row(
                                wh,
                                layer,
                                PropertyName::Y,
                                &layer.image.y,
                                &self.y_context,
                            )
                        }),
                        ratio(1.0, |wh| {
                            self.render_f32_based_graph_row(
                                wh,
                                layer,
                                PropertyName::Width,
                                &layer.image.width,
                                &self.width_context,
                            )
                        }),
                        ratio(1.0, |wh| {
                            self.render_f32_based_graph_row(
                                wh,
                                layer,
                                PropertyName::Height,
                                &layer.image.height,
                                &self.height_context,
                            )
                        }),
                        ratio(1.0, |wh| {
                            self.render_f32_based_graph_row(
                                wh,
                                layer,
                                PropertyName::RotationAngle,
                                &layer.image.rotation_angle,
                                &self.rotation_angle_context,
                            )
                        }),
                        ratio(1.0, |wh| {
                            self.render_f32_based_graph_row(
                                wh,
                                layer,
                                PropertyName::Opacity,
                                &layer.image.opacity,
                                &self.opacity_context,
                            )
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
                props.playback_time,
                self.context.start_at,
                self.context.time_per_pixel,
            ),
        ])
    }
}

impl GraphWindow {
    fn render_f32_based_graph_row<TValue: KeyframeValue + Copy + From<f32> + Into<f32>>(
        &self,
        wh: Wh<f32>,
        layer: &Layer,
        property_name: PropertyName,
        graph: &KeyframeGraph<TValue>,
        property_context: &PropertyContext<TValue>,
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
                    property_context,
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
    fn render_one_zero_graph_row(
        &self,
        wh: Wh<f32>,
        layer: &Layer,
        property_name: PropertyName,
        graph: &KeyframeGraph<OneZero>,
        property_context: &PropertyContext<OneZero>,
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
                    property_context,
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
                PropertyName::RotationAngle => "Rotate",
                PropertyName::Opacity => "Opacity",
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
                let mouse_global_xy = namui::system::mouse::mouse_position();
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

                if namui::system::keyboard::any_code_press([
                    namui::Code::ShiftLeft,
                    namui::Code::ShiftRight,
                ]) {
                    namui::event::send(Event::GraphShiftMouseWheel {
                        delta: PixelSize::from(event.delta_xy.y),
                    })
                } else if namui::system::keyboard::any_code_press([
                    namui::Code::AltLeft,
                    namui::Code::AltRight,
                ]) {
                    namui::event::send(Event::GraphAltMouseWheel {
                        delta: PixelSize::from(event.delta_xy.y),
                        mouse_local_xy,
                    })
                } else if namui::system::keyboard::any_code_press([
                    namui::Code::ControlLeft,
                    namui::Code::ControlRight,
                ]) {
                    namui::event::send(Event::GraphCtrlMouseWheel {
                        delta: PixelSize::from(event.delta_xy.y),
                        mouse_local_xy,
                        property_name,
                        row_wh: wh,
                    })
                }
            })
    })
}
