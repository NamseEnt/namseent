use super::*;
mod selected_layer_timeline;

impl TimelineWindow {
    pub(crate) fn render(&self, props: Props) -> RenderingTree {
        let background_for_event =
            simple_rect(props.wh, Color::TRANSPARENT, 0.0, Color::TRANSPARENT).attach_event(
                |builder| {
                    builder
                        .on_wheel(move |event| {
                            let managers = namui::managers();
                            let mouse_global_xy = managers.mouse_manager.mouse_position();
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
                            if managers
                                .keyboard_manager
                                .any_code_press([Code::ShiftLeft, Code::ShiftRight])
                            {
                                namui::event::send(Event::ShiftWheel {
                                    delta: event.delta_xy.y,
                                });
                            } else if managers
                                .keyboard_manager
                                .any_code_press([Code::AltLeft, Code::AltRight])
                            {
                                namui::event::send(Event::AltWheel {
                                    delta: event.delta_xy.y,
                                    anchor_xy: mouse_local_xy,
                                });
                            }
                        })
                        .on_mouse_down(|event| {
                            namui::event::send(Event::TimelineClicked {
                                mouse_local_xy: event.local_xy,
                            })
                        })
                        .on_mouse_move_in(|event| {
                            namui::event::send(Event::TimelineMouseMoveIn {
                                mouse_local_xy: event.local_xy,
                            })
                        })
                },
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
        ])
    }
}
