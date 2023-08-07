use super::*;
use crate::{web::SelectionDirection, *};

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

        ctx.web_event(|event| {
            let on_event = self.on_event.lock().unwrap().take().unwrap();
            match event {
                web::WebEvent::MouseDown { event } => {
                    on_event(Event::MouseDown {
                        event: get_mouse_event(ctx, &done, event, MouseEventType::Down),
                    });
                }
                web::WebEvent::MouseMove { event } => {
                    on_event(Event::MouseMove {
                        event: get_mouse_event(ctx, &done, event, MouseEventType::Move),
                    });
                }
                web::WebEvent::MouseUp { event } => {
                    on_event(Event::MouseUp {
                        event: get_mouse_event(ctx, &done, event, MouseEventType::Up),
                    });
                }
                web::WebEvent::Wheel { event } => {
                    on_event(Event::Wheel {
                        event: WheelEvent {
                            delta_xy: event.delta_xy,
                            mouse_local_xy: ctx.local_xy(event.mouse_xy),
                            is_stop_propagation: Default::default(), // TODO
                        },
                    });
                }
                web::WebEvent::HashChange { new_url, old_url } => {
                    on_event(Event::HashChange {
                        new_url: new_url.clone(),
                        old_url: old_url.clone(),
                    });
                }
                &web::WebEvent::SelectionChange {
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
                web::WebEvent::KeyDown { code } => {
                    // TODO
                    // on_event(Event::KeyDown {
                    //     event: KeyboardEvent {
                    //         code: todo!(),
                    //         pressing_codes: todo!(),
                    //     },
                    // });
                }
                web::WebEvent::KeyUp { code } => {
                    // TODO
                    // on_event(Event::KeyUp {
                    //     event: KeyboardEvent {
                    //         code: todo!(),
                    //         pressing_codes: todo!(),
                    //     },
                    // });
                }
                web::WebEvent::Blur => {
                    on_event(Event::Blur);
                }
                web::WebEvent::VisibilityChange => {
                    on_event(Event::VisibilityChange);
                }
                &web::WebEvent::Resize { width, height } => {
                    on_event(Event::Resize { width, height });
                }
                &web::WebEvent::AsyncFunction { id } => {
                    on_event(Event::AsyncFunction { id });
                }
                &web::WebEvent::TextInputTextUpdated {
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
                &web::WebEvent::TextInputKeyDown {
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
            }
        });

        return done;

        fn get_mouse_event<'a>(
            ctx: &'a RenderCtx,
            done: &'a RenderDone,
            event: &'a RawMouseEvent,
            mouse_event_type: MouseEventType,
        ) -> MouseEvent<'a> {
            MouseEvent {
                local_xy: Box::new(|| ctx.local_xy(event.xy)),
                is_local_xy_in: Box::new(|| {
                    done.rendering_tree.is_xy_in(
                        ctx.local_xy(event.xy),
                        &[
                                //TODO
                            ],
                    )
                }),
                global_xy: event.xy,
                pressing_buttons: event.pressing_buttons.clone(),
                button: event.button,
                event_type: mouse_event_type,
                is_stop_propagation: Default::default(), // TODO
            }
        }
    }
}

pub enum Event<'a> {
    MouseDown {
        event: MouseEvent<'a>,
    },
    MouseMove {
        event: MouseEvent<'a>,
    },
    MouseUp {
        event: MouseEvent<'a>,
    },
    Wheel {
        event: WheelEvent,
    },
    HashChange {
        new_url: String,
        old_url: String,
    },
    // Drop {
    //     dataTransfer: Option<web_sys::DataTransfer>,
    //     x: usize,
    //     y: usize,
    // },
    SelectionChange {
        selection_direction: SelectionDirection,
        selection_start: usize,
        selection_end: usize,
        text: String,
    },
    KeyDown {
        event: KeyboardEvent,
    },
    KeyUp {
        event: KeyboardEvent,
    },
    Blur,
    VisibilityChange,
    Resize {
        width: usize,
        height: usize,
    },
    AsyncFunction {
        id: usize,
    },
    TextInputTextUpdated {
        text: String,
        selection_direction: SelectionDirection,
        selection_start: usize,
        selection_end: usize,
    },
    TextInputKeyDown {
        code: Code,
        text: String,
        selection_direction: SelectionDirection,
        selection_start: usize,
        selection_end: usize,
        is_composing: bool,
    },
}
