use super::*;
use crate::{
    color,
    pages::sequence_edit_page::{
        atom::{UpdateCgFile, CG_FILES_ATOM, SEQUENCE_ATOM},
        components::{cg_upload::create_cg, image_upload::create_image},
    },
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
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            cut,
            wh,
            is_selecting_target,
            on_event,
            on_internal_event,
            project_id,
        } = self;
        let cut_id = cut.id;

        ctx.component(
            simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND).attach_event(
                |event| {
                    match event {
                        namui::Event::MouseDown { event } => {
                            if event.is_local_xy_in() {
                                event.stop_propagation();
                                if event.button == Some(MouseButton::Right) {
                                    on_internal_event(InternalEvent::MouseRightButtonDown {
                                        global_xy: event.global_xy,
                                        cut_id,
                                    })
                                }
                            }
                        }
                        namui::Event::KeyDown { event } => {
                            if event.code == Code::KeyV && namui::keyboard::ctrl_press() {
                                spawn_local(async move {
                                    if let Ok(buffers) = clipboard::read_image_buffers().await {
                                        for png_bytes in buffers {
                                            add_new_image(project_id, cut_id, png_bytes);
                                        }
                                    }

                                    if let Ok(items) = clipboard::read().await {
                                        for item in items {
                                            if item.types().iter().any(|type_| {
                                                type_ == "web application/luda-editor-cg+json"
                                            }) {
                                                if let Ok(cg) = item
                                                    .get_type("web application/luda-editor-cg+json")
                                                    .await
                                                    .map(|graphic_bytes| {
                                                        serde_json::from_slice::<ScreenCg>(
                                                            &graphic_bytes,
                                                        )
                                                        .unwrap()
                                                    })
                                                {
                                                    add_cg(cut_id, cg);
                                                }
                                            }
                                        }
                                    }
                                });
                            } else if event.code == Code::ArrowUp
                                || event.code == Code::ArrowDown
                                || event.code == Code::Tab && !is_selecting_target
                            {
                                if event.code == Code::ArrowUp
                                    || (namui::keyboard::shift_press() && event.code == Code::Tab)
                                {
                                    on_event(Event::MoveCutRequest {
                                        up_down: UpDown::Up,
                                    })
                                } else {
                                    on_event(Event::MoveCutRequest {
                                        up_down: UpDown::Down,
                                    })
                                };
                            }
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
            Err(_error) => {
                todo!();
            }
        };
    });
}

fn add_new_cg(project_id: Uuid, cut_id: Uuid, psd_name: String, psd_bytes: Vec<u8>) {
    spawn_local(async move {
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
            Err(_error) => {
                todo!();
            }
        }
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
