use super::*;
use rpc::data::ScreenCg;

impl CutEditor {
    pub fn background_with_event(&self, props: &Props, cut: &Cut) -> namui::RenderingTree {
        let cut_id = cut.id();
        let selected_target = self.selected_target;
        let prev_cut_id = prev_cut_id(&props, cut_id);
        let next_cut_id = next_cut_id(&props, cut_id);

        simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND).attach_event(
            |build| {
                build
                    .on_file_drop(move |event: FileDropEvent| {
                        let file = event.files[0].clone();
                        spawn_local(async move {
                            let content = file.content().await;
                            match file.name().ends_with(".psd") {
                                true => namui::event::send(Event::AddNewCg {
                                    psd_bytes: content.into(),
                                    psd_name: file.name().trim_end_matches(".psd").to_string(),
                                    cut_id,
                                }),
                                false => namui::event::send(Event::AddNewImage {
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
                                        namui::event::send(Event::AddNewImage { png_bytes, cut_id })
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
                                                namui::event::send(Event::AddCg { cut_id, cg })
                                            }
                                        }
                                    }
                                }
                            });
                        } else if event.code == Code::ArrowUp
                            || event.code == Code::ArrowDown
                            || event.code == Code::Tab && selected_target.is_none()
                        {
                            let move_cut_id = if event.code == Code::ArrowUp
                                || (namui::keyboard::shift_press() && event.code == Code::Tab)
                            {
                                prev_cut_id
                            } else {
                                next_cut_id
                            };
                            let Some(move_cut_id) = move_cut_id else {
                                return
                            };
                            namui::event::send(Event::MoveCutRequest {
                                cut_id: move_cut_id,
                                to_prev: event.code == Code::ArrowUp,
                                focused: false,
                            })
                        }
                    })
                    .on_mouse_down_in(move |event: MouseEvent| {
                        event.stop_propagation();
                        if event.button == Some(MouseButton::Right) {
                            namui::event::send(InternalEvent::MouseRightButtonDown {
                                global_xy: event.global_xy,
                                cut_id,
                            })
                        }
                    });
            },
        )
    }
}
