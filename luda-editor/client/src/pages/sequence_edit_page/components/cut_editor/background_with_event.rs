use super::*;
use crate::{
    color,
    pages::sequence_edit_page::{atom::SEQUENCE_ATOM, components::image_upload::create_image},
};
use namui_prebuilt::*;
use rpc::data::{CutUpdateAction, ScreenCg, ScreenGraphic, ScreenImage};

#[namui::component]
pub struct BackgroundWithEvent<'a> {
    pub cut: &'a Cut,
    pub wh: Wh<Px>,
    pub is_selecting_target: bool,
    pub prev_cut_id: Option<Uuid>,
    pub next_cut_id: Option<Uuid>,
    pub(super) on_event: Box<dyn 'a + Fn(Event)>,
    pub(super) on_internal_event: callback!('a, InternalEvent),
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
            prev_cut_id,
            next_cut_id,
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
                        _ => {}
                    };
                    // .on_file_drop(move |event: FileDropEvent| {
                    //     let file = event.files[0].clone();
                    //     spawn_local(async move {
                    //         let content = file.content().await;
                    //         match file.name().ends_with(".psd") {
                    //             true => on_event(Event::AddNewCg {
                    //                 psd_bytes: content.into(),
                    //                 psd_name: file.name().trim_end_matches(".psd").to_string(),
                    //                 cut_id,
                    //             }),
                    //             false => on_event(Event::AddNewImage {
                    //                 png_bytes: content.into(),
                    //                 cut_id,
                    //             }),
                    //         }
                    //     });
                    // })
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
            Err(error) => {
                // namui::event::send(InternalEvent::Error(format!(
                //     "create_image {}",
                //     error.to_string()
                // )));
            }
        };
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
