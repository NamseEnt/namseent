use super::*;
use crate::color;
use namui_prebuilt::*;
use rpc::data::ScreenCg;

#[namui::component]
pub struct BackgroundWithEvent {
    pub cut: Cut,
    pub wh: Wh<Px>,
    pub is_selecting_target: bool,
    pub prev_cut_id: Option<Uuid>,
    pub next_cut_id: Option<Uuid>,
    pub on_event: &'a dyn Fn(Event),
}

pub enum Event {
    MoveCutRequest {
        up_down: UpDown,
    },
    AddNewCg {
        psd_bytes: Vec<u8>,
        psd_name: String,
        cut_id: Uuid,
    },
    AddNewImage {
        png_bytes: Vec<u8>,
        cut_id: Uuid,
    },
    AddCg {
        cut_id: Uuid,
        cg: ScreenCg,
    },
}

impl Component for BackgroundWithEvent {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            ref cut,
            wh,
            is_selecting_target,
            prev_cut_id,
            next_cut_id,
            ref on_event,
        } = self;
        let cut_id = cut.id;
        let on_event = on_event.clone();
        ctx.use_children(|ctx| {
            ctx.add(
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND).attach_event(
                    |build| {
                        build
                            .on_file_drop(move |event: FileDropEvent| {
                                let file = event.files[0].clone();
                                spawn_local(async move {
                                    let content = file.content().await;
                                    match file.name().ends_with(".psd") {
                                        true => on_event.call(Event::AddNewCg {
                                            psd_bytes: content.into(),
                                            psd_name: file
                                                .name()
                                                .trim_end_matches(".psd")
                                                .to_string(),
                                            cut_id,
                                        }),
                                        false => on_event.call(Event::AddNewImage {
                                            png_bytes: content.into(),
                                            cut_id,
                                        }),
                                    }
                                });
                            })
                            .on_key_down(move |event: KeyboardEvent| {
                                if event.code == Code::KeyV && namui::keyboard::ctrl_press() {
                                    spawn_local(async move {
                                        if let Ok(buffers) = clipboard::read_image_buffers().await {
                                            for png_bytes in buffers {
                                                on_event
                                                    .call(Event::AddNewImage { png_bytes, cut_id })
                                            }
                                        }

                                        if let Ok(items) = clipboard::read().await {
                                            for item in items {
                                                if item.types().iter().any(|type_| {
                                                    type_ == "web application/luda-editor-cg+json"
                                                }) {
                                                    if let Ok(cg) = item
                                                        .get_type(
                                                            "web application/luda-editor-cg+json",
                                                        )
                                                        .await
                                                        .map(|graphic_bytes| {
                                                            serde_json::from_slice::<ScreenCg>(
                                                                &graphic_bytes,
                                                            )
                                                            .unwrap()
                                                        })
                                                    {
                                                        on_event.call(Event::AddCg { cut_id, cg })
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
                                        || (namui::keyboard::shift_press()
                                            && event.code == Code::Tab)
                                    {
                                        on_event.call(Event::MoveCutRequest {
                                            up_down: UpDown::Up,
                                        })
                                    } else {
                                        on_event.call(Event::MoveCutRequest {
                                            up_down: UpDown::Down,
                                        })
                                    };
                                }
                            })
                            .on_mouse_down_in(move |event: MouseEvent| {
                                event.stop_propagation();
                                if event.button == Some(MouseButton::Right) {
                                    on_event.call(InternalEvent::MouseRightButtonDown {
                                        global_xy: event.global_xy,
                                        cut_id,
                                    })
                                }
                            });
                    },
                ),
            );
        })
    }
}
