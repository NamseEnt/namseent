use super::*;
use crate::*;

pub(crate) fn attach_event<'a, C: 'a + Component>(
    component: C,
    on_event: impl 'a + FnOnce(Event),
) -> AttachEvent<'a, C> {
    AttachEvent {
        component,
        on_event: Mutex::new(Some(Box::new(on_event))),
    }
}

pub struct AttachEvent<'a, C: Component> {
    component: C,
    on_event: Mutex<Option<Box<dyn 'a + FnOnce(Event)>>>,
}
impl<'a, C: 'a + Component> StaticType for AttachEvent<'a, C> {}
impl<'a, C: 'a + Component> Debug for AttachEvent<'a, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttachEvent")
            .field("component", &self.component)
            .finish()
    }
}
impl<'b, C: 'b + Component> Component for AttachEvent<'b, C> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        ctx.component(self.component);
        let done = ctx.done();

        if ctx.event_handling_disabled() {
            return done;
        }

        ctx.on_raw_event(|raw_event| {
            let on_event = self.on_event.lock().unwrap().take().unwrap();
            invoke_on_event(
                on_event,
                raw_event,
                ctx.inverse_matrix(),
                &done.rendering_tree,
            );
        });

        return done;
    }
}

pub(crate) fn invoke_on_event(
    on_event: impl FnOnce(Event<'_>),
    raw_event: &RawEvent,
    inverse_matrix: Matrix3x3,
    rendering_tree: &RenderingTree,
) {
    match raw_event {
        RawEvent::MouseDown { event } => {
            on_event(Event::MouseDown {
                event: get_mouse_event(
                    inverse_matrix,
                    &rendering_tree,
                    event,
                    MouseEventType::Down,
                ),
            });
        }
        RawEvent::MouseMove { event } => {
            on_event(Event::MouseMove {
                event: get_mouse_event(
                    inverse_matrix,
                    &rendering_tree,
                    event,
                    MouseEventType::Move,
                ),
            });
        }
        RawEvent::MouseUp { event } => {
            on_event(Event::MouseUp {
                event: get_mouse_event(inverse_matrix, &rendering_tree, event, MouseEventType::Up),
            });
        }
        RawEvent::Wheel { event } => {
            on_event(Event::Wheel {
                event: WheelEvent {
                    delta_xy: event.delta_xy,
                    mouse_local_xy: inverse_matrix.transform_xy(event.mouse_xy),
                    is_stop_propagation: Default::default(), // TODO
                },
            });
        }
        RawEvent::HashChange { new_url, old_url } => {
            on_event(Event::HashChange {
                new_url: new_url.clone(),
                old_url: old_url.clone(),
            });
        }
        &RawEvent::SelectionChange {
            selection_direction,
            selection_start,
            selection_end,
            ref text,
        } => {
            on_event(Event::SelectionChange {
                selection_direction,
                selection_start,
                selection_end,
                text: text.clone(),
            });
        }
        &RawEvent::KeyDown {
            code,
            ref pressing_code_set,
        } => {
            on_event(Event::KeyDown {
                event: KeyboardEvent {
                    code,
                    pressing_codes: pressing_code_set.clone(),
                },
            });
        }
        &RawEvent::KeyUp {
            code,
            ref pressing_code_set,
        } => {
            on_event(Event::KeyUp {
                event: KeyboardEvent {
                    code,
                    pressing_codes: pressing_code_set.clone(),
                },
            });
        }
        RawEvent::Blur => {
            on_event(Event::Blur);
        }
        RawEvent::VisibilityChange => {
            on_event(Event::VisibilityChange);
        }
        &RawEvent::ScreenResize { wh } => {
            on_event(Event::ScreenResize { wh });
        }
        &RawEvent::TextInputTextUpdated {
            ref text,
            selection_direction,
            selection_start,
            selection_end,
        } => {
            on_event(Event::TextInputTextUpdated {
                text: text.clone(),
                selection_direction,
                selection_start,
                selection_end,
            });
        }
        &RawEvent::TextInputKeyDown {
            code,
            ref text,
            selection_direction,
            selection_start,
            selection_end,
            is_composing,
        } => {
            on_event(Event::TextInputKeyDown {
                code,
                text: text.clone(),
                selection_direction,
                selection_start,
                selection_end,
                is_composing,
            });
        }
        RawEvent::FileDrop { data_transfer, xy } => todo!(),
    }
}

fn get_mouse_event<'a>(
    inverse_matrix: Matrix3x3,
    rendering_tree: &'a RenderingTree,
    raw_mouse_event: &'a RawMouseEvent,
    mouse_event_type: MouseEventType,
) -> MouseEvent<'a> {
    MouseEvent {
        local_xy: Box::new(move || inverse_matrix.transform_xy(raw_mouse_event.xy)),
        is_local_xy_in: Box::new(move || {
            rendering_tree.xy_in(inverse_matrix.transform_xy(raw_mouse_event.xy))
        }),
        global_xy: raw_mouse_event.xy,
        pressing_buttons: raw_mouse_event.pressing_buttons.clone(),
        button: raw_mouse_event.button,
        event_type: mouse_event_type,
        is_stop_propagation: Default::default(), // TODO
    }
}
