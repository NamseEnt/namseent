use super::*;
use crate::zoom::zoom_time_per_px;
use namui::animation::*;

impl TimelineWindow {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| match event {
            &Event::ShiftWheel { delta } => {
                self.start_at += Px::from(delta) * self.time_per_px;
            }
            &Event::AltWheel { delta, anchor_xy } => {
                let time_at_mouse_position =
                    self.start_at + Px::from(anchor_xy.x) * self.time_per_px;

                let next_time_per_px = zoom_time_per_px(self.time_per_px, delta.into());

                let next_start_at =
                    time_at_mouse_position - Px::from(anchor_xy.x) * next_time_per_px;

                self.time_per_px = next_time_per_px;
                self.start_at = next_start_at;
            }
            &Event::TimelineLeftMouseDown { mouse_local_xy } => {
                if self.dragging.is_none() {
                    self.dragging = Some(Dragging::Background {
                        last_mouse_local_xy: mouse_local_xy,
                    });

                    let time = self.start_at + Px::from(mouse_local_xy.x) * self.time_per_px;
                    self.set_playback_time(time);
                }
            }
            &Event::TimelineMouseMoveIn { mouse_local_xy } => {
                self.handle_timeline_dragging(mouse_local_xy);
            }
            &Event::KeyframeMouseDown {
                ref point_id,
                anchor_xy,
                mouse_local_xy,
                keyframe_time,
                layer_id,
            } => {
                namui::event::send(crate::time_point_editor::Event::ChangEditingTarget(Some(
                    EditingTarget::Keyframe {
                        point_id: point_id.clone(),
                        layer_id,
                    },
                )));
                self.set_playback_time(keyframe_time);
                if self.dragging.is_none() {
                    if let Some(action_ticket) =
                        self.animation_history
                            .try_set_action(DraggingKeyframeAction {
                                layer_id,
                                point_id: point_id.clone(),
                                drag_end_x: mouse_local_xy.x.into(),
                                anchor_x: Px::from(anchor_xy.x),
                                time_per_px: self.time_per_px,
                                start_at: self.start_at,
                            })
                    {
                        self.dragging = Some(Dragging::Keyframe { action_ticket });
                    }
                }
            }
            &Event::TimelineRightMouseDown {
                mouse_local_xy,
                ref selected_layer_id,
            } => {
                if self.dragging.is_none() {
                    self.dragging = None;
                    if let Some(layer_id) = selected_layer_id {
                        self.crate_new_keyframe(mouse_local_xy, *layer_id);
                    }
                }
            }
            &Event::TimelineDeleteKeyDown {
                ref selected_layer_id,
                playback_time,
            } => {
                if let Some(selected_layer_id) = selected_layer_id {
                    struct DeleteKeyframeAction {
                        layer_id: namui::Uuid,
                        time: Time,
                    }
                    impl Act<Animation> for DeleteKeyframeAction {
                        fn act(
                            &self,
                            state: &Animation,
                        ) -> Result<Animation, Box<dyn std::error::Error>> {
                            let mut animation = state.clone();
                            if let Some(layer) = animation
                                .layers
                                .iter_mut()
                                .find(|layer| layer.id.eq(&self.layer_id))
                            {
                                delete_point(layer, self.time);
                                Ok(animation)
                            } else {
                                Err("layer not found".into())
                            }
                        }
                    }

                    if let Some(action_ticket) =
                        self.animation_history.try_set_action(DeleteKeyframeAction {
                            layer_id: selected_layer_id.clone(),
                            time: playback_time,
                        })
                    {
                        self.animation_history.act(action_ticket).unwrap();
                    }
                }
            }
            Event::TimelineSpaceKeyDown {
                selected_layer_id,
                editing_target,
            } => {
                let was_playing = self.playing_status.is_playing();

                self.playing_status.toggle_play();

                if was_playing {
                    if let Some(EditingTarget::Keyframe { point_id, layer_id }) = editing_target {
                        if selected_layer_id.as_ref().eq(&Some(layer_id)) {
                            self.move_playback_time_to_point(*layer_id, *point_id);
                        }
                    }
                }
            }
            &Event::LineMouseDown { point_id, layer_id } => {
                namui::event::send(crate::time_point_editor::Event::ChangEditingTarget(Some(
                    EditingTarget::Line { point_id, layer_id },
                )));
            }
            Event::MouseLeftDownOutOfEditingTargetButInWindow => {
                namui::log!("MouseDownOutOfKeyframeButInWindow");
                namui::event::send(crate::time_point_editor::Event::ChangEditingTarget(None));
            }
            Event::MouseUp => {
                if let Some(Dragging::Keyframe { action_ticket }) = self.dragging {
                    self.animation_history.act(action_ticket).unwrap();
                }
                self.dragging = None;
            }
        });
    }
    fn move_playback_time_to_point(&mut self, layer_id: Uuid, point_id: Uuid) {
        let animation = self.animation_history.get_preview();

        let selected_layer = animation
            .layers
            .iter()
            .find(|layer| layer.id.eq(&layer_id))
            .unwrap();

        let point = selected_layer
            .image
            .image_keyframe_graph
            .get_point(point_id)
            .unwrap();

        let time = point.time;

        self.playing_status.set_playback_time(time);
    }
    fn handle_timeline_dragging(&mut self, mouse_local_xy: Xy<Px>) {
        if self.dragging.is_none() {
            return;
        }

        let dragging = self.dragging.as_mut().unwrap();

        match dragging {
            Dragging::Background {
                last_mouse_local_xy,
            } => {
                let delta = mouse_local_xy - *last_mouse_local_xy;
                self.start_at -= Px::from(delta.x) * self.time_per_px;
                *last_mouse_local_xy = mouse_local_xy;
            }
            Dragging::Keyframe { ref action_ticket } => {
                let mut anchor_x = Px::from(0.0);
                self.animation_history
                    .update_action(*action_ticket, |action: &mut DraggingKeyframeAction| {
                        action.time_per_px = self.time_per_px;
                        action.start_at = self.start_at;
                        action.drag_end_x = mouse_local_xy.x.into();

                        anchor_x = action.anchor_x;
                    })
                    .unwrap();

                self.set_playback_time(
                    self.start_at + (Px::from(mouse_local_xy.x) - anchor_x) * self.time_per_px,
                );
            }
        }
    }
    fn crate_new_keyframe(&mut self, mouse_local_xy: Xy<Px>, layer_id: Uuid) {
        struct CreateNewKeyframeAction {
            layer_id: namui::Uuid,
            time: Time,
        }
        impl Act<Animation> for CreateNewKeyframeAction {
            fn act(&self, state: &Animation) -> Result<Animation, Box<dyn std::error::Error>> {
                let mut animation = state.clone();

                if let Some(layer) = animation
                    .layers
                    .iter_mut()
                    .find(|layer| layer.id.eq(&self.layer_id))
                {
                    let time = self.time;

                    let new_keyframe = {
                        let mut most_nearest_left_keyframe = None;

                        let mut iter = layer
                            .image
                            .image_keyframe_graph
                            .get_point_line_tuples()
                            .peekable();

                        while let Some((point, _)) = iter.next() {
                            if let Some((next_point, _)) = iter.peek() {
                                if next_point.time > time {
                                    most_nearest_left_keyframe = Some(point.value.clone());
                                    break;
                                }
                            } else {
                                most_nearest_left_keyframe = Some(point.value.clone());
                            }
                        }

                        most_nearest_left_keyframe.unwrap_or(ImageKeyframe {
                            matrix: namui::Matrix3x3::identity(),
                            opacity: 1.0.into(),
                        })
                    };

                    add_new_point(&mut layer.image.image_keyframe_graph, time, new_keyframe);

                    Ok(animation)
                } else {
                    Err("layer not found".into())
                }
            }
        }

        let time = self.start_at + Px::from(mouse_local_xy.x) * self.time_per_px;
        if let Some(action_ticket) = self
            .animation_history
            .try_set_action(CreateNewKeyframeAction { layer_id, time })
        {
            self.animation_history.act(action_ticket).unwrap();

            self.set_playback_time(time);
        }
    }
}

fn move_point(
    layer: &mut Layer,
    point_id: Uuid,
    to_time: Time,
) -> Result<(), Box<dyn std::error::Error>> {
    let point = layer
        .image
        .image_keyframe_graph
        .get_point_mut(point_id)
        .ok_or(format!("point nod fount {}", point_id))?;

    point.time = to_time;

    Ok(())
}

fn add_new_point(graph: &mut ImageKeyframeGraph, time: Time, default_value: ImageKeyframe) {
    graph.put(
        KeyframePoint::new(time, default_value),
        animation::ImageInterpolation::AllLinear,
    );
}

fn delete_point(layer: &mut Layer, time: Time) {
    layer.image.image_keyframe_graph.delete_by_time(time)
}

struct DraggingKeyframeAction {
    layer_id: namui::Uuid,
    point_id: namui::Uuid,
    drag_end_x: Px,
    anchor_x: Px,
    start_at: Time,
    time_per_px: TimePerPx,
}
impl Act<Animation> for DraggingKeyframeAction {
    fn act(&self, state: &Animation) -> Result<Animation, Box<dyn std::error::Error>> {
        let mut animation = state.clone();
        if let Some(layer) = animation
            .layers
            .iter_mut()
            .find(|layer| layer.id.eq(&self.layer_id))
        {
            let to_time = self.start_at + (self.drag_end_x - self.anchor_x) * self.time_per_px;

            move_point(layer, self.point_id, to_time)?;

            Ok(animation)
        } else {
            Err("layer not found".into())
        }
    }
}
