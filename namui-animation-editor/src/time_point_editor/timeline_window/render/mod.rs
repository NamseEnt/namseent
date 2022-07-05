use super::*;
mod selected_layer_timeline;

impl TimelineWindow {
    pub(crate) fn render(&self, props: Props) -> RenderingTree {
        let background_for_event =
            simple_rect(props.wh, Color::TRANSPARENT, px(0.0), Color::TRANSPARENT)
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

                            let mouse_local_xy = mouse_global_xy - table_xy;

                            if mouse_local_xy.x < px(0.0)
                                || props.wh.width < mouse_local_xy.x
                                || mouse_local_xy.y < px(0.0)
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

        let playback_time_x = (self.get_playback_time() - self.start_at) / self.time_per_px;
        let playback_time_line = namui::path(
            PathBuilder::new()
                .move_to(playback_time_x.into(), px(0.0))
                .line_to(playback_time_x.into(), props.wh.height),
            PaintBuilder::new()
                .set_style(PaintStyle::Stroke)
                .set_color(Color::RED)
                .set_stroke_width(px(1.0)),
        );

        render([
            background_for_event,
            vertical([
                ratio(1.0, |wh| {
                    crate::time_ruler::render(&crate::time_ruler::Props {
                        rect: Rect::from_xy_wh(Xy::single(px(0.0)), wh),
                        start_at: self.start_at,
                        time_per_px: self.time_per_px,
                    })
                }),
                ratio(2.0, |wh| {
                    // TODO: Timeline for other layers
                    simple_rect(wh, Color::BLACK, px(1.0), Color::grayscale_f01(0.5))
                }),
                ratio(7.0, |wh| self.render_selected_layer_timeline(wh, &props)),
            ])(props.wh),
            playback_time_line,
        ])
    }
}
