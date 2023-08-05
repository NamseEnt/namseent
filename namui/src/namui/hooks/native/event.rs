use super::*;
use crate::{web::SelectionDirection, *};

// pub(crate) fn on_mouse_down_in<'a>(
//     component: impl 'a + Component,
//     on_mouse_down_in: impl 'a + FnOnce(MouseEvent),
// ) -> OnMouseDownIn<'a> {
//     OnMouseDownIn {
//         component: Box::new(component),
//         on_mouse_down_in: Mutex::new(Some(Box::new(on_mouse_down_in))),
//     }
// }

// pub struct OnMouseDownIn<'a> {
//     component: Box<dyn 'a + Component>,
//     on_mouse_down_in: Mutex<Option<Box<dyn 'a + FnOnce(MouseEvent)>>>,
// }
// impl StaticType for OnMouseDownIn<'_> {}
// impl Debug for OnMouseDownIn<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("OnMouseDownIn")
//             .field("component", &self.component)
//             .finish()
//     }
// }
// impl Component for OnMouseDownIn<'_> {
//     fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
//         let done = ctx.return_(self.component.as_ref());

//         ctx.web_event(|event| {
//             if let crate::web::WebEvent::MouseDown { event } = event {
//                 let local_xy = ctx.matrix.transform_xy(event.xy);
//                 if done.rendering_tree.is_xy_in(
//                     local_xy,
//                     &[
//                     //TODO
//                 ],
//                 ) {
//                     let on_mouse_down_in = self.on_mouse_down_in.lock().unwrap().take().unwrap();
//                     on_mouse_down_in(MouseEvent {
//                         local_xy: Box::new(|| local_xy),
//                         global_xy: event.xy,
//                         pressing_buttons: event.pressing_buttons.clone(),
//                         button: event.button,
//                         event_type: MouseEventType::Down,
//                         is_stop_propagation: Default::default(), // TODO
//                     })
//                 }
//             }
//         });

//         done
//     }
// }

// pub(crate) fn on_mouse<'a>(
//     component: impl 'a + Component,
//     on_mouse: impl 'a + FnOnce(MouseEvent),
// ) -> OnMouse<'a> {
//     OnMouse {
//         component: Box::new(component),
//         on_mouse: Mutex::new(Some(Box::new(on_mouse))),
//     }
// }

// pub struct OnMouse<'a> {
//     component: Box<dyn 'a + Component>,
//     on_mouse: Mutex<Option<Box<dyn 'a + FnOnce(MouseEvent)>>>,
// }
// impl StaticType for OnMouse<'_> {}
// impl Debug for OnMouse<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("OnMouse")
//             .field("component", &self.component)
//             .finish()
//     }
// }
// impl Component for OnMouse<'_> {
//     fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
//         let done = ctx.return_(self.component.as_ref());

//         ctx.web_event(|event| {
//             if let crate::web::WebEvent::MouseDown { event } = event {
//                 let local_xy = ctx.matrix.transform_xy(event.xy);
//                 let on_mouse = self.on_mouse.lock().unwrap().take().unwrap();
//                 on_mouse(MouseEvent {
//                     local_xy: Box::new(|| local_xy),
//                     global_xy: event.xy,
//                     pressing_buttons: event.pressing_buttons.clone(),
//                     button: event.button,
//                     event_type: MouseEventType::Down,
//                     is_stop_propagation: Default::default(), // TODO
//                 })
//             }
//         });

//         done
//     }
// }

pub(crate) fn on_event<'a>(
    component: impl 'a + Component,
    on_event: impl 'a + FnOnce(Event),
) -> OnEvent<'a> {
    OnEvent {
        component: Box::new(component),
        on_event: Mutex::new(Some(Box::new(on_event))),
    }
}

pub struct OnEvent<'a> {
    component: Box<dyn 'a + Component>,
    on_event: Mutex<Option<Box<dyn 'a + FnOnce(Event)>>>,
}
impl StaticType for OnEvent<'_> {}
impl Debug for OnEvent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnEvent")
            .field("component", &self.component)
            .finish()
    }
}
impl Component for OnEvent<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let done = ctx.return_(self.component.as_ref());

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
                            mouse_local_xy: ctx.matrix.lock().unwrap().transform_xy(event.mouse_xy),
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
                    on_event(Event::KeyDown { code: code.clone() });
                }
                web::WebEvent::KeyUp { code } => {
                    on_event(Event::KeyUp { code: code.clone() });
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
                local_xy: Box::new(|| ctx.matrix.lock().unwrap().transform_xy(event.xy)),
                is_local_xy_in: Box::new(|| {
                    done.rendering_tree.is_xy_in(
                        ctx.matrix.lock().unwrap().transform_xy(event.xy),
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
        code: String,
    },
    KeyUp {
        code: String,
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
