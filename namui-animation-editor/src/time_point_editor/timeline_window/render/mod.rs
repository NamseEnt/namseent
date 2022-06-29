use super::*;
mod selected_layer_timeline;

impl TimelineWindow {
    pub(crate) fn render(&self, props: Props) -> RenderingTree {
        let background_for_event =
            simple_rect(props.wh, Color::TRANSPARENT, 0.0, Color::TRANSPARENT)
                .with_id(&self.window_id)
                .attach_event(|builder| {
                    let playback_time = self.get_playback_time();
                    let selected_layer_id = props.selected_layer_id.clone();
                    builder
                        .on_wheel(move |event| {
                            let mouse_global_xy = namui::mouse::position();
                            let table_xy = event
                                .namui_context
                                .get_rendering_tree_xy(event.target)
                                .expect("ERROR: fail to get rendering_tree_xy");

                            let mouse_local_xy = Xy {
                                x: mouse_global_xy.x as f32 - table_xy.x,
                                y: mouse_global_xy.y as f32 - table_xy.y,
                            };

                            if mouse_local_xy.x < 0.0
                                || props.wh.width < mouse_local_xy.x
                                || mouse_local_xy.y < 0.0
                                || props.wh.height < mouse_local_xy.y
                            {
                                return;
                            }
                            if namui::keyboard::any_code_press([Code::ShiftLeft, Code::ShiftRight])
                            {
                                namui::event::send(Event::ShiftWheel {
                                    delta: event.delta_xy.y,
                                });
                            } else if namui::keyboard::any_code_press([
                                Code::AltLeft,
                                Code::AltRight,
                            ]) {
                                namui::event::send(Event::AltWheel {
                                    delta: event.delta_xy.y,
                                    anchor_xy: mouse_local_xy,
                                });
                            }
                        })
                        .on_mouse_down(move |event| {
                            if event.button.is_none() {
                                return;
                            }
                            let button = event.button.unwrap();
                            match button {
                                MouseButton::Left => {
                                    namui::event::send(Event::TimelineLeftMouseDown {
                                        mouse_local_xy: event.local_xy,
                                    })
                                }
                                MouseButton::Right => {
                                    namui::event::send(Event::TimelineRightMouseDown {
                                        mouse_local_xy: event.local_xy,
                                        selected_layer_id: selected_layer_id.clone(),
                                    })
                                }
                                _ => {}
                            }
                        })
                        .on_mouse_move_in(|event| {
                            namui::event::send(Event::TimelineMouseMoveIn {
                                mouse_local_xy: event.local_xy,
                            })
                        });

                    let selected_layer_id = props.selected_layer_id.clone();
                    builder.on_key_down(move |event| {
                        if event.code == Code::Delete {
                            namui::event::send(Event::TimelineDeleteKeyDown {
                                selected_layer_id: selected_layer_id.clone(),
                                playback_time,
                            });
                        } else if event.code == Code::Space {
                            namui::event::send(Event::TimelineSpaceKeyDown);
                        }
                    });
                });

        let playback_time_x = (self.get_playback_time() - self.start_at) / self.time_per_pixel;
        let playback_time_line = namui::path(
            PathBuilder::new()
                .move_to(playback_time_x.into(), 0.0)
                .line_to(playback_time_x.into(), props.wh.height),
            PaintBuilder::new()
                .set_style(PaintStyle::Stroke)
                .set_color(Color::RED)
                .set_stroke_width(1.0),
        );

        render([
            background_for_event,
            vertical([
                ratio(1.0, |wh| {
                    crate::time_ruler::render(&crate::time_ruler::Props {
                        xywh: XywhRect {
                            x: 0.0,
                            y: 0.0,
                            width: wh.width.into(),
                            height: wh.height.into(),
                        },
                        start_at: self.start_at,
                        time_per_pixel: self.time_per_pixel,
                    })
                }),
                ratio(2.0, |wh| {
                    // TODO: Timeline for other layers
                    simple_rect(wh, Color::BLACK, 1.0, Color::grayscale_f01(0.5))
                }),
                ratio(7.0, |wh| self.render_selected_layer_timeline(wh, &props)),
            ])(props.wh),
            playback_time_line,
        ])
    }
}
