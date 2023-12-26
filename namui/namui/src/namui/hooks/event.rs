use crate::*;
use std::sync::{atomic::AtomicBool, Arc};
#[cfg(target_family = "wasm")]
use web_sys::DataTransfer;

pub(crate) fn invoke_on_event(
    tree_ctx: &TreeContext,
    on_event: impl FnOnce(Event<'_>),
    raw_event: &RawEvent,
    inverse_matrix: Matrix3x3,
    rendering_tree: &RenderingTree,
    global_xy_clip_in: impl ClipIn,
) {
    if tree_ctx
        .is_stop_event_propagation
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        return;
    }
    let is_stop_event_propagation = tree_ctx.is_stop_event_propagation.clone();

    match raw_event {
        RawEvent::MouseDown { event } => {
            let event = get_mouse_event(
                inverse_matrix,
                rendering_tree,
                event,
                MouseEventType::Down,
                global_xy_clip_in,
                is_stop_event_propagation,
            );

            on_event(Event::MouseDown { event });
        }
        RawEvent::MouseMove { event } => {
            on_event(Event::MouseMove {
                event: get_mouse_event(
                    inverse_matrix,
                    rendering_tree,
                    event,
                    MouseEventType::Move,
                    global_xy_clip_in,
                    is_stop_event_propagation,
                ),
            });
        }
        RawEvent::MouseUp { event } => {
            on_event(Event::MouseUp {
                event: get_mouse_event(
                    inverse_matrix,
                    rendering_tree,
                    event,
                    MouseEventType::Up,
                    global_xy_clip_in,
                    is_stop_event_propagation,
                ),
            });
        }
        RawEvent::Wheel { event } => {
            on_event(Event::Wheel {
                event: WheelEvent {
                    delta_xy: event.delta_xy,
                    local_xy: Box::new(move || inverse_matrix.transform_xy(event.mouse_xy)),
                    is_local_xy_in: Box::new(move || {
                        global_xy_clip_in.clip_in(event.mouse_xy)
                            && BoundingBox::xy_in(
                                rendering_tree,
                                inverse_matrix.transform_xy(event.mouse_xy),
                            )
                    }),
                    is_stop_event_propagation,
                },
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
        RawEvent::KeyDown { event } => {
            on_event(Event::KeyDown {
                event: KeyboardEvent {
                    code: event.code,
                    pressing_codes: &event.pressing_codes,
                    prevent_default: &event.prevent_default,
                    is_stop_event_propagation,
                },
            });
        }
        RawEvent::KeyUp { event } => {
            on_event(Event::KeyUp {
                event: KeyboardEvent {
                    code: event.code,
                    pressing_codes: &event.pressing_codes,
                    prevent_default: &event.prevent_default,
                    is_stop_event_propagation,
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
                text: text.as_str(),
                selection_direction,
                selection_start,
                selection_end,
            });
        }
        RawEvent::TextInputKeyDown { event } => {
            on_event(Event::TextInputKeyDown {
                event: TextInputKeyDownEvent {
                    code: event.code,
                    text: &event.text,
                    selection_direction: event.selection_direction,
                    selection_start: event.selection_start,
                    selection_end: event.selection_end,
                    is_composing: event.is_composing,
                    prevent_default: &event.prevent_default,
                    is_stop_event_propagation,
                },
            });
        }
        #[cfg(target_family = "wasm")]
        &RawEvent::FileDrop {
            ref data_transfer,
            xy,
        } => {
            on_event(Event::DragAndDrop {
                event: get_file_drop_event(
                    inverse_matrix,
                    rendering_tree,
                    data_transfer,
                    xy,
                    global_xy_clip_in,
                ),
            });
        }
        RawEvent::ScreenRedraw => {
            on_event(Event::ScreenRedraw);
        }
    }
}

fn get_mouse_event<'a>(
    inverse_matrix: Matrix3x3,
    rendering_tree: &'a RenderingTree,
    raw_mouse_event: &'a RawMouseEvent,
    mouse_event_type: MouseEventType,
    global_xy_clip_in: impl ClipIn + 'a,
    is_stop_event_propagation: Arc<AtomicBool>,
) -> MouseEvent<'a> {
    MouseEvent {
        local_xy: Box::new(move || inverse_matrix.transform_xy(raw_mouse_event.xy)),
        is_local_xy_in: Box::new(move || {
            global_xy_clip_in.clip_in(raw_mouse_event.xy)
                && rendering_tree.xy_in(inverse_matrix.transform_xy(raw_mouse_event.xy))
        }),
        global_xy: raw_mouse_event.xy,
        pressing_buttons: raw_mouse_event.pressing_buttons.clone(),
        button: raw_mouse_event.button,
        event_type: mouse_event_type,
        prevent_default: &raw_mouse_event.prevent_default,
        is_stop_event_propagation,
    }
}

#[cfg(target_family = "wasm")]
fn get_file_drop_event<'a>(
    inverse_matrix: Matrix3x3,
    rendering_tree: &'a RenderingTree,
    data_transfer: &Option<DataTransfer>,
    global_xy: Xy<Px>,
    global_xy_clip_in: impl ClipIn + 'a,
) -> FileDropEvent<'a> {
    let files = data_transfer.as_ref().map_or(vec![], |data_transfer| {
        let items = data_transfer.items();
        let mut files = Vec::with_capacity(items.length() as usize);
        for index in 0..items.length() {
            let web_file = items.get(index).unwrap().get_as_file().unwrap().unwrap();
            let file = File::new(web_file);
            files.push(file);
        }
        files
    });

    FileDropEvent {
        is_local_xy_in: Box::new(move || {
            global_xy_clip_in.clip_in(global_xy)
                && rendering_tree.xy_in(inverse_matrix.transform_xy(global_xy))
        }),
        local_xy: Box::new(move || inverse_matrix.transform_xy(global_xy)),
        global_xy,
        files,
    }
}
