#[cfg(test)]
mod test;

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
                .parent_stack()
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
                        let path_xy_in = path.xy_in(global_xy);
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

        /*
        Q. Which local do you mean?
        1 2 3 4 5 6 7 8 9
        ^------------------                    <= global
            ^-------------- ctx.compose        <= parent local
                  ^-------- ctx.translate
                      ^---- ctx.add
                        ^-- ctx.attach_event   <= local
        */

        let to_local_xy = |xy| apply_commands_to_xy(xy, self.full_stack.iter());
        let to_parent_local_xy = |xy| apply_commands_to_xy(xy, self.parent_stack());

        let xy_in = |global_xy: Xy<Px>| -> bool {
            let Some(bounding_box) = self.rt_container.iter().bounding_box() else {
                return false;
            };
            let parent_local_xy = to_parent_local_xy(global_xy);
            if !bounding_box.is_xy_inside(parent_local_xy) {
                return false;
            }

            self.rt_container.iter().any(|rt| rt.xy_in(parent_local_xy))
        };

        match raw_event {
            RawEvent::MouseDown { event } => {
                let event = MouseEvent {
                    local_xy: &move || to_local_xy(event.xy),
                    is_local_xy_in: &move || is_global_xy_clip_in(event.xy) && xy_in(event.xy),
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
                    is_local_xy_in: &move || is_global_xy_clip_in(event.xy) && xy_in(event.xy),
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
                    is_local_xy_in: &move || is_global_xy_clip_in(event.xy) && xy_in(event.xy),
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
                            is_global_xy_clip_in(event.mouse_xy) && xy_in(event.mouse_xy)
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

fn apply_commands_to_xy<'a>(
    mut target_xy: Xy<Px>,
    commands: impl Iterator<Item = &'a ComposeCommand> + 'a,
) -> Xy<Px> {
    let original_xy = target_xy;
    for command in commands {
        match command {
            ComposeCommand::Translate { xy } => target_xy -= xy,
            ComposeCommand::Absolute { xy } => target_xy = original_xy - xy,
            ComposeCommand::Rotate { angle } => {
                target_xy = TransformMatrix::from_rotate(-angle).transform_xy(target_xy);
            }
            ComposeCommand::Scale { scale_xy } => {
                target_xy = TransformMatrix::from_scale(1.0 / scale_xy.x, 1.0 / scale_xy.y)
                    .transform_xy(target_xy);
            }
            ComposeCommand::Clip { .. }
            | ComposeCommand::OnTop
            | ComposeCommand::MouseCursor { .. } => {}
        }
    }

    target_xy
}
