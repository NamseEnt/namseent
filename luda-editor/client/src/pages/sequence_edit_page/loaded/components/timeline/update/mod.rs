use super::*;
use rpc::data::{ImageClip, ImageClipAddress};

impl Timeline {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::OpenContextMenu(context_menu) => {
                    self.context_menu = Some(context_menu.clone());
                }
                Event::CloseContextMenu => {
                    self.context_menu = None;
                }
                Event::NewImageClip => {
                    self.editor_history_system.mutate_cut(
                        &self.selected_sequence_id,
                        &self.selected_cut_id,
                        |cut| {
                            cut.image_clips.push(ImageClip::new(5.sec()));
                        },
                    );
                }
                Event::SelectImageClip { .. } => {}
                Event::DeselectImageClip => {}
            }
        } else if let Some(event) = event.downcast_ref::<resizable_clip::Event>() {
            match event {
                resizable_clip::Event::MouseDown {
                    clip_id,
                    clicked_part,
                    ctrl_key_pressed,
                    shift_key_pressed,
                    global_mouse_x,
                } => {
                    match clicked_part {
                        resizable_clip::ResizableClipBodyPart::Sash(_direction) => {
                            self.clip_sash_dragging = Some(ClipSashDragging {
                                clip_id: clip_id.clone(),
                                global_mouse_x: *global_mouse_x,
                                start_global_mouse_x: *global_mouse_x,
                            });
                        }
                        resizable_clip::ResizableClipBodyPart::Body => {
                            // self.clip_id_to_check_as_click = None;

                            if *ctrl_key_pressed {
                                if self.selected_clip_ids.contains(clip_id) {
                                    self.deselect_clips([clip_id]);
                                } else {
                                    self.multi_select_clip(clip_id);
                                }
                            }
                            // else if *shift_key_pressed
                            //     && self.is_clip_in_same_track_with_selected_clips(clip_id)
                            // {
                            //     self.select_all_to_time(click_in_time);
                            // }
                            else if !self.selected_clip_ids.contains(clip_id) {
                                self.select_only_this_clip(clip_id);
                            } else {
                                // self.clip_id_to_check_as_click = Some(clip_id.to_string());
                            }

                            if self.selected_clip_ids.len() > 0 {
                                namui::event::send(Event::SelectImageClip {
                                    sequence_id: self.selected_sequence_id.clone(),
                                    cut_id: self.selected_cut_id.clone(),
                                    image_clip_ids: self.selected_clip_ids.clone(),
                                })
                            } else {
                                namui::event::send(Event::DeselectImageClip)
                            }
                        }
                    }
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::AnimationFrame
                | NamuiEvent::MouseDown(_)
                | NamuiEvent::KeyDown(_)
                | NamuiEvent::KeyUp(_)
                | NamuiEvent::ScreenResize(_)
                | NamuiEvent::Wheel(_)
                | NamuiEvent::DeepLinkOpened(_) => {}
                NamuiEvent::MouseUp(event) => {
                    if let Some(clip_sash_dragging) = &mut self.clip_sash_dragging {
                        let x_delta = event.xy.x - clip_sash_dragging.start_global_mouse_x;
                        let delta_duration = x_delta * self.time_per_px;

                        let selected_clip_id =
                            self.selected_clip_ids.iter().next().unwrap().clone();

                        let image_clip_address = ImageClipAddress {
                            sequence_id: self.selected_sequence_id.clone(),
                            cut_id: self.selected_cut_id.clone(),
                            image_clip_id: selected_clip_id,
                        };

                        self.editor_history_system
                            .mutate_image_clip(&image_clip_address, |clip| {
                                clip.duration += delta_duration;
                            });

                        self.clip_sash_dragging = None;
                    }
                }
                NamuiEvent::MouseMove(event) => {
                    if let Some(clip_sash_dragging) = &mut self.clip_sash_dragging {
                        clip_sash_dragging.global_mouse_x = event.xy.x;
                    }
                }
            }
        }
    }
    fn select_only_this_clip(&mut self, clip_id: Uuid) {
        self.selected_clip_ids.clear();
        self.selected_clip_ids.insert(clip_id.to_string());
    }
    fn deselect_clips<AsRefStr: AsRef<str>>(
        &mut self,
        clip_ids: impl IntoIterator<Item = AsRefStr>,
    ) {
        for clip_id in clip_ids {
            self.selected_clip_ids.remove(clip_id.as_ref());
        }
    }
    fn multi_select_clip(&mut self, clip_id: Uuid) {
        if self.selected_clip_ids.is_empty() {
            self.selected_clip_ids.insert(clip_id.to_string());
        } else if !self.selected_clip_ids.contains(clip_id) {
            // TODO
            // if selected_clip_track.get_id() != selecting_clip_track.get_id() {
            //     self.selected_clip_ids.clear();
            // }
            self.selected_clip_ids.insert(clip_id.to_string());
        }
    }
}
