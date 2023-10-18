use super::*;
use crate::{
    app::notification::{push_notification, remove_notification, Notification},
    clipboard::TryReadLudaEditorClipboardItem,
    color,
    components::{cg_upload::create_cg, image_upload::create_image},
    pages::sequence_edit_page::atom::{UpdateCgFile, CG_FILES_ATOM, NAME_QUICK_SLOT},
};
use namui_prebuilt::*;
use rpc::data::{CutUpdateAction, ScreenCg, ScreenGraphic, ScreenImage};

#[namui::component]
pub struct BackgroundWithEvent<'a> {
    pub cut: &'a Cut,
    pub wh: Wh<Px>,
    pub is_selecting_target: bool,
    pub(super) on_event: Box<dyn 'a + Fn(Event)>,
    pub(super) on_internal_event: &'a dyn Fn(InternalEvent),
    pub project_id: Uuid,
}

pub(super) enum Event {
    MoveCutRequest { up_down: UpDown },
}

impl Component for BackgroundWithEvent<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            cut,
            wh,
            is_selecting_target,
            on_event,
            on_internal_event,
            project_id,
        } = self;
        let cut_id = cut.id;
        let (name_quick_slot, _) = ctx.atom(&NAME_QUICK_SLOT);

        let handle_paste = |event: &KeyboardEvent| {
            if !(event.code == Code::KeyV && namui::keyboard::ctrl_press()) {
                return;
            }
            spawn_local(async move {
                if let Ok(buffers) = clipboard::read_image_buffers().await {
                    for png_bytes in buffers {
                        add_new_image(project_id, cut_id, png_bytes);
                    }
                }

                if let Ok(items) = clipboard::read().await {
                    for item in items {
                        if let Some(cg) = item.try_read_from_clipboard().await {
                            add_cg(cut_id, cg);
                        };
                    }
                }
            });
        };

        let handle_key_move = |event: &KeyboardEvent| {
            if !(event.code == Code::ArrowUp
                || event.code == Code::ArrowDown
                || event.code == Code::Tab && !is_selecting_target)
            {
                return;
            }

            let up_down = if event.code == Code::ArrowUp
                || (namui::keyboard::shift_press() && event.code == Code::Tab)
            {
                UpDown::Up
            } else {
                UpDown::Down
            };

            on_event(Event::MoveCutRequest { up_down });
        };

        let handle_undo_redo = |event: &KeyboardEvent| {
            let ctrl_press = namui::keyboard::ctrl_press();

            let undo_redo = if ctrl_press && event.code == Code::KeyY
                || ctrl_press && namui::keyboard::shift_press() && event.code == Code::KeyZ
            {
                UndoRedo::Redo
            } else if ctrl_press && event.code == Code::KeyZ {
                UndoRedo::Undo
            } else {
                return;
            };

            SEQUENCE_ATOM.mutate(move |sequence| match undo_redo {
                UndoRedo::Undo => sequence.undo(),
                UndoRedo::Redo => sequence.redo(),
            });

            enum UndoRedo {
                Undo,
                Redo,
            }
        };

        let handle_name_quick_slot_shortcut = |event: &KeyboardEvent| {
            let ctrl_press = namui::keyboard::ctrl_press();
            if !ctrl_press {
                return;
            }
            let quick_slot_index = match event.code {
                Code::Digit1 => 0,
                Code::Digit2 => 1,
                Code::Digit3 => 2,
                Code::Digit4 => 3,
                Code::Digit5 => 4,
                _ => {
                    return;
                }
            };
            event.prevent_default();
            let Some(name) = name_quick_slot.get_name(quick_slot_index).cloned() else {
                push_notification(Notification::error(format!(
                    "Name quick slot {quick_slot_index} not registered. Please register it first"
                )));
                return;
            };

            SEQUENCE_ATOM.mutate(move |sequence| {
                sequence.update_cut(cut_id, CutUpdateAction::ChangeCharacterName { name })
            });
        };

        ctx.component(
            simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND).attach_event(
                |event| {
                    match event {
                        namui::Event::MouseDown { event } => {
                            if event.is_local_xy_in() && event.button == Some(MouseButton::Right) {
                                event.stop_propagation();
                                on_internal_event(InternalEvent::MouseRightButtonDown {
                                    global_xy: event.global_xy,
                                    cut_id,
                                })
                            }
                        }
                        namui::Event::KeyDown { event } => {
                            handle_paste(&event);
                            handle_key_move(&event);
                            handle_undo_redo(&event);
                            handle_name_quick_slot_shortcut(&event);
                        }
                        namui::Event::DragAndDrop { event } => {
                            if event.is_local_xy_in() {
                                for file in event.files {
                                    spawn_local(async move {
                                        let content = file.content().await;
                                        match file.name().ends_with(".psd") {
                                            true => {
                                                let psd_bytes = content.into();
                                                let psd_name = file
                                                    .name()
                                                    .trim_end_matches(".psd")
                                                    .to_string();
                                                add_new_cg(project_id, cut_id, psd_name, psd_bytes);
                                            }
                                            false => {
                                                let png_bytes = content.into();
                                                add_new_image(project_id, cut_id, png_bytes);
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        _ => {}
                    };
                },
            ),
        );

        ctx.done()
    }
}

fn add_new_image(project_id: Uuid, cut_id: Uuid, png_bytes: Vec<u8>) {
    spawn_local(async move {
        let notification_id = push_notification(
            Notification::info("Uploading image...".to_string()).set_loading(true),
        );
        match create_image(project_id, png_bytes).await {
            Ok(image_id) => {
                SEQUENCE_ATOM.mutate(move |sequence| {
                    sequence.update_cut(
                        cut_id,
                        CutUpdateAction::PushScreenGraphic {
                            graphic_index: uuid(),
                            screen_graphic: ScreenGraphic::Image(ScreenImage::new(image_id)),
                        },
                    )
                });
            }
            Err(error) => {
                push_notification(Notification::error(format!(
                    "Failed to upload image: {error}"
                )));
            }
        };
        remove_notification(notification_id);
    });
}

fn add_new_cg(project_id: Uuid, cut_id: Uuid, psd_name: String, psd_bytes: Vec<u8>) {
    spawn_local(async move {
        let notification_id = push_notification(
            Notification::info(format!("Uploading CG {psd_name}...")).set_loading(true),
        );
        match create_cg(project_id, psd_name, psd_bytes).await {
            Ok(cg_file) => {
                CG_FILES_ATOM.mutate({
                    let cg_file = cg_file.clone();
                    move |cg_files| {
                        cg_files.update_file(cg_file);
                    }
                });

                let graphic_index = uuid();

                SEQUENCE_ATOM.mutate({
                    let screen_cg = ScreenCg::new(&cg_file);
                    move |sequence| {
                        sequence.update_cut(
                            cut_id,
                            CutUpdateAction::PushScreenGraphic {
                                graphic_index,
                                screen_graphic: ScreenGraphic::Cg(screen_cg),
                            },
                        )
                    }
                });
            }
            Err(error) => {
                push_notification(Notification::error(format!("Failed to upload CG: {error}")));
            }
        }
        remove_notification(notification_id);
    });
}

fn add_cg(cut_id: Uuid, cg: ScreenCg) {
    SEQUENCE_ATOM.mutate(move |sequence| {
        sequence.update_cut(
            cut_id,
            CutUpdateAction::PushScreenGraphic {
                graphic_index: uuid(),
                screen_graphic: ScreenGraphic::Cg(cg.clone()),
            },
        )
    });
}
