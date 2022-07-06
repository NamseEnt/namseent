mod render_graph;

use std::fmt::Display;

use super::*;
use namui::animation::KeyframeValue;
use render_graph::*;

impl GraphWindow {
    pub fn render(&self, props: Props) -> RenderingTree {
        if props.layer.is_none() {
            return simple_rect(props.wh, Color::WHITE, px(1.0), Color::BLACK);
        }
        let layer = props.layer.unwrap();

        render([
            vertical([
                fixed(px(20.0), |wh| {
                    time_ruler::render(&time_ruler::Props {
                        start_at: self.context.start_at,
                        time_per_px: self.context.time_per_px,
                        rect: Rect::from_xy_wh(Xy::single(px(0.0)), wh),
                    })
                }),
                ratio(1.0, |wh| {
                    vertical([
                        ratio(1.0, |wh| {
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
                                &layer.image.width_percent,
                                &self.width_context,
                            )
                        }),
                        ratio(1.0, |wh| {
                            self.render_f32_based_graph_row(
                                wh,
                                layer,
                                PropertyName::Height,
                                &layer.image.height_percent,
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
                        });
                    })
                }),
            ])(props.wh),
            render_playback_time_line(
                props.wh,
                props.playback_time,
                self.context.start_at,
                self.context.time_per_px,
            ),
        ])
    }
}

impl GraphWindow {
    fn render_f32_based_graph_row<
        TValue: KeyframeValue + Copy + FromPrimitive + ToPrimitive + Display,
    >(
        &self,
        wh: Wh<Px>,
        layer: &Layer,
        property_name: PropertyName,
        graph: &KeyframeGraph<TValue>,
        property_context: &PropertyContext<TValue>,
    ) -> RenderingTree {
        render_graph_row(
            wh,
            property_name,
            (
                graph,
                Context {
                    start_at: self.context.start_at,
                    time_per_px: self.context.time_per_px,
                    property_context,
                    mouse_local_xy: self.mouse_over_row.as_ref().and_then(|mouse_over_row| {
                        if mouse_over_row.property_name == property_name {
                            Some(mouse_over_row.mouse_xy_in_row)
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
    wh: Wh<Px>,
    playback_time: Time,
    start_at: Time,
    time_per_px: TimePerPx,
) -> RenderingTree {
    let x = (playback_time - start_at) / time_per_px;
    let color = Color::RED;
    let path = namui::PathBuilder::new()
        .move_to(x, px(0.0))
        .line_to(x, wh.height);
    let paint = namui::PaintBuilder::new()
        .set_color(color)
        .set_style(namui::PaintStyle::Stroke)
        .set_stroke_width(px(1.0));

    namui::path(path, paint)
}

fn render_graph_row(
    wh: Wh<Px>,
    property_name: PropertyName,
    render_graph: impl RenderGraph,
) -> RenderingTree {
    let label_wh = Wh {
        width: px(30.0),
        height: wh.height / 8.0,
    };
    let label = render([
        simple_rect(label_wh, Color::BLACK, px(1.0), Color::WHITE),
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
        simple_rect(wh, Color::WHITE, px(1.0), Color::BLACK),
        render_graph.render(wh),
        label,
    ])
    .attach_event(move |builder| {
        builder
            .on_mouse_move_in(move |event| {
                namui::event::send(Event::GraphMouseMoveIn {
                    property_name,
                    mouse_xy_in_row: event.local_xy,
                })
            })
            .on_mouse_down(move |event| match event.button {
                Some(MouseButton::Left) => namui::event::send(Event::GraphMouseLeftDown {
                    property_name,
                    mouse_local_xy: event.local_xy,
                }),
                _ => {}
            })
            .on_wheel(move |event| {
                let mouse_global_xy = namui::mouse::position();
                let row_xy = event
                    .namui_context
                    .get_rendering_tree_xy(event.target)
                    .expect("ERROR: fail to get rendering_tree_xy");

                let mouse_local_xy = Xy {
                    x: mouse_global_xy.x - row_xy.x,
                    y: mouse_global_xy.y - row_xy.y,
                };

                if mouse_local_xy.x < px(0.0)
                    || wh.width < mouse_local_xy.x
                    || mouse_local_xy.y < px(0.0)
                    || wh.height < mouse_local_xy.y
                {
                    return;
                }

                if namui::keyboard::any_code_press([
                    namui::Code::ShiftLeft,
                    namui::Code::ShiftRight,
                ]) {
                    namui::event::send(Event::GraphShiftMouseWheel {
                        delta: Px::from(event.delta_xy.y),
                    })
                } else if namui::keyboard::any_code_press([
                    namui::Code::AltLeft,
                    namui::Code::AltRight,
                ]) {
                    namui::event::send(Event::GraphAltMouseWheel {
                        delta: Px::from(event.delta_xy.y),
                        mouse_local_xy,
                    })
                } else if namui::keyboard::any_code_press([
                    namui::Code::ControlLeft,
                    namui::Code::ControlRight,
                ]) {
                    namui::event::send(Event::GraphCtrlMouseWheel {
                        delta: Px::from(event.delta_xy.y),
                        mouse_local_xy,
                        property_name,
                        row_wh: wh,
                    })
                } else {
                    namui::event::send(Event::GraphMouseWheel {
                        delta: Px::from(event.delta_xy.y),
                        property_name,
                    })
                }
            })
            .on_key_down(move |event| {
                namui::event::send(Event::KeyboardKeyDown {
                    code: event.code,
                    row_height: wh.height,
                })
            });
    })
}
