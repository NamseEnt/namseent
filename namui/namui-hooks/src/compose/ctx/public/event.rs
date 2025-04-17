use super::*;
use crate::*;

impl ComposeCtx<'_, '_> {
    pub fn on_raw_event(&self, on_event: impl FnOnce(&RawEvent)) -> &Self {
        if self.world.is_stop_event_propagation() {
            return self;
        }

        if let Some(raw_event) = self.world.raw_event.as_ref() {
            on_event(raw_event);
        }

        self
    }
    pub fn attach_event(&self, on_event: impl FnOnce(Event<'_>)) -> &Self {
        let Some(raw_event) = self.world.raw_event.as_ref() else {
            return self;
        };

        if self.world.is_stop_event_propagation() {
            return self;
        }

        let is_global_xy_clip_in = |mut global_xy: Xy<Px>| -> bool {
            if self
                .full_stack
                .iter()
                .all(|command| !matches!(command, ComposeCommand::Clip { .. }))
            {
                return true;
            }

            let original_xy = global_xy;
            for command in self.full_stack.iter() {
                match command {
                    ComposeCommand::Translate { xy } => global_xy -= xy,
                    ComposeCommand::Absolute { xy } => global_xy = original_xy - xy,
                    ComposeCommand::Clip { path, clip_op } => {
                        let path_xy_in = path.xy_in(self.world.sk_calculate, global_xy);
                        match clip_op {
                            ClipOp::Intersect => {
                                if !path_xy_in {
                                    return false;
                                }
                            }
                            ClipOp::Difference => {
                                if path_xy_in {
                                    return false;
                                }
                            }
                        }
                    }
                    ComposeCommand::Rotate { angle } => {
                        global_xy = TransformMatrix::from_rotate(-angle).transform_xy(global_xy);
                    }
                    ComposeCommand::Scale { scale_xy } => {
                        global_xy = TransformMatrix::from_scale(1.0 / scale_xy.x, 1.0 / scale_xy.y)
                            .transform_xy(global_xy);
                    }
                    ComposeCommand::OnTop | ComposeCommand::MouseCursor { .. } => {}
                }
            }

            true
        };

        let to_local_xy = |mut global_xy: Xy<Px>| -> Xy<Px> {
            let original_xy = global_xy;
            for command in self.full_stack.iter() {
                match command {
                    ComposeCommand::Translate { xy } => global_xy -= xy,
                    ComposeCommand::Absolute { xy } => global_xy = original_xy - xy,
                    ComposeCommand::Rotate { angle } => {
                        global_xy = TransformMatrix::from_rotate(-angle).transform_xy(global_xy);
                    }
                    ComposeCommand::Scale { scale_xy } => {
                        global_xy = TransformMatrix::from_scale(1.0 / scale_xy.x, 1.0 / scale_xy.y)
                            .transform_xy(global_xy);
                    }
                    ComposeCommand::Clip { .. }
                    | ComposeCommand::OnTop
                    | ComposeCommand::MouseCursor { .. } => {}
                }
            }

            global_xy
        };

        let bounding_box_xy_in = |xy: Xy<Px>| -> bool {
            let Some(bounding_box) = self
                .rt_container
                .iter()
                .bounding_box(self.world.sk_calculate)
            else {
                return false;
            };
            let xy = to_local_xy(xy);
            bounding_box.is_xy_inside(xy)
        };

        match raw_event {
            RawEvent::MouseDown { event } => {
                let event = MouseEvent {
                    local_xy: &move || to_local_xy(event.xy),
                    is_local_xy_in: &move || {
                        is_global_xy_clip_in(event.xy) && bounding_box_xy_in(event.xy)
                    },
                    global_xy: event.xy,
                    pressing_buttons: &event.pressing_buttons,
                    button: event.button,
                    event_type: MouseEventType::Down,
                    is_stop_event_propagation: &self.world.is_stop_event_propagation,
                };

                on_event(Event::MouseDown { event });
            }
            RawEvent::MouseMove { event } => {
                let event = MouseEvent {
                    local_xy: &move || to_local_xy(event.xy),
                    is_local_xy_in: &move || {
                        is_global_xy_clip_in(event.xy) && bounding_box_xy_in(event.xy)
                    },
                    global_xy: event.xy,
                    pressing_buttons: &event.pressing_buttons,
                    button: event.button,
                    event_type: MouseEventType::Move,
                    is_stop_event_propagation: &self.world.is_stop_event_propagation,
                };

                on_event(Event::MouseMove { event });
            }
            RawEvent::MouseUp { event } => {
                let event = MouseEvent {
                    local_xy: &move || to_local_xy(event.xy),
                    is_local_xy_in: &move || {
                        is_global_xy_clip_in(event.xy) && bounding_box_xy_in(event.xy)
                    },
                    global_xy: event.xy,
                    pressing_buttons: &event.pressing_buttons,
                    button: event.button,
                    event_type: MouseEventType::Up,
                    is_stop_event_propagation: &self.world.is_stop_event_propagation,
                };

                on_event(Event::MouseUp { event });
            }
            RawEvent::Wheel { event } => {
                on_event(Event::Wheel {
                    event: WheelEvent {
                        delta_xy: event.delta_xy,
                        local_xy: &move || to_local_xy(event.mouse_xy),
                        is_local_xy_in: &move || {
                            is_global_xy_clip_in(event.mouse_xy)
                                && bounding_box_xy_in(event.mouse_xy)
                        },
                        is_stop_event_propagation: &self.world.is_stop_event_propagation,
                    },
                });
            }
            RawEvent::KeyDown { event } => {
                on_event(Event::KeyDown {
                    event: KeyboardEvent {
                        code: event.code,
                        pressing_codes: &event.pressing_codes,
                        is_stop_event_propagation: &self.world.is_stop_event_propagation,
                    },
                });
            }
            RawEvent::KeyUp { event } => {
                on_event(Event::KeyUp {
                    event: KeyboardEvent {
                        code: event.code,
                        pressing_codes: &event.pressing_codes,
                        is_stop_event_propagation: &self.world.is_stop_event_propagation,
                    },
                });
            }
            RawEvent::Blur => on_event(Event::Blur),
            RawEvent::VisibilityChange => on_event(Event::VisibilityChange),
            &RawEvent::ScreenResize { wh } => on_event(Event::ScreenResize { wh }),
            RawEvent::ScreenRedraw => on_event(Event::ScreenRedraw),
            RawEvent::TextInput { event } => on_event(Event::TextInput { event }),
            RawEvent::TextInputKeyDown { event } => on_event(Event::TextInputKeyDown { event }),
            RawEvent::TextInputSelectionChange { event } => {
                on_event(Event::TextInputSelectionChange { event })
            }
        }

        self
    }
}
