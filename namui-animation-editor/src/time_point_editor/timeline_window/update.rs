use super::*;
use crate::zoom::zoom_time_per_pixel;
use namui::animation::{KeyframeGraph, KeyframePoint, KeyframeValue};

impl TimelineWindow {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                &Event::ShiftWheel { delta } => {
                    self.start_at += PixelSize::from(delta) * self.time_per_pixel;
                }
                &Event::AltWheel { delta, anchor_xy } => {
                    let time_at_mouse_position =
                        self.start_at + PixelSize::from(anchor_xy.x) * self.time_per_pixel;

                    let next_time_per_pixel =
                        zoom_time_per_pixel(self.time_per_pixel, delta.into());

                    let next_start_at =
                        time_at_mouse_position - PixelSize::from(anchor_xy.x) * next_time_per_pixel;

                    self.time_per_pixel = next_time_per_pixel;
                    self.start_at = next_start_at;
                }
                &Event::TimelineLeftMouseDown { mouse_local_xy } => {
                    if self.dragging.is_none() {
                        self.selected_point_ids = None;
                        self.dragging = Some(Dragging::Background {
                            last_mouse_local_xy: mouse_local_xy,
                        });
                    }

                    let time =
                        self.start_at + PixelSize::from(mouse_local_xy.x) * self.time_per_pixel;
                    namui::event::send(crate::time_point_editor::Event::UpdatePlaybackTime(time));
                }
                &Event::TimelineMouseMoveIn { mouse_local_xy } => {
                    self.handle_timeline_dragging(mouse_local_xy);
                }
                Event::KeyframeMouseDown {
                    point_ids,
                    anchor_xy,
                    keyframe_time,
                    mouse_local_xy,
                } => {
                    self.selected_point_ids = Some(point_ids.clone());
                    let layer_id = self.selected_layer_id.as_ref().unwrap();
                    if self.dragging.is_none() {
                        if let Some(action_ticket) =
                            self.animation_history
                                .try_set_action(DraggingKeyframeAction {
                                    layer_id: layer_id.clone(),
                                    point_ids: point_ids.clone(),
                                    drag_end_x: mouse_local_xy.x.into(),
                                    anchor_x: PixelSize::from(anchor_xy.x),
                                    time_per_pixel: self.time_per_pixel,
                                    start_at: self.start_at,
                                })
                        {
                            self.dragging = Some(Dragging::Keyframe { action_ticket });
                        }
                    }
                }
                &Event::TimelineRightMouseDown { mouse_local_xy } => {
                    if self.selected_point_ids.is_none() && self.dragging.is_none() {
                        self.selected_point_ids = None;
                        self.dragging = None;
                        self.crate_new_keyframe(mouse_local_xy);
                    }
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseUp(_) => {
                    if let Some(Dragging::Keyframe { action_ticket }) = self.dragging {
                        self.animation_history.act(action_ticket).unwrap();
                    }
                    self.dragging = None;
                }
                NamuiEvent::KeyDown(event) => match event.code {
                    Code::Delete => {
                        if let Some(selected_point_ids) = &self.selected_point_ids {
                            let selected_layer_id = self.selected_layer_id.as_ref().unwrap();

                            struct DeleteKeyframe {
                                layer_id: String,
                                point_ids: Vec<String>,
                            }
                            impl Act<Animation> for DeleteKeyframe {
                                fn act(
                                    &self,
                                    state: &Animation,
                                ) -> Result<Animation, Box<dyn std::error::Error>>
                                {
                                    let mut animation = state.clone();
                                    if let Some(layer) = animation
                                        .layers
                                        .iter_mut()
                                        .find(|layer| layer.id.eq(&self.layer_id))
                                    {
                                        for point_id in &self.point_ids {
                                            delete_point(layer, &point_id);
                                        }
                                        Ok(animation)
                                    } else {
                                        Err("layer not found".into())
                                    }
                                }
                            }

                            if let Some(action_ticket) =
                                self.animation_history.try_set_action(DeleteKeyframe {
                                    layer_id: selected_layer_id.clone(),
                                    point_ids: selected_point_ids.clone(),
                                })
                            {
                                self.animation_history.act(action_ticket).unwrap();

                                self.selected_point_ids = None;
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    fn handle_timeline_dragging(&mut self, mouse_local_xy: Xy<f32>) {
        if self.dragging.is_none() {
            return;
        }

        let dragging = self.dragging.as_mut().unwrap();

        match dragging {
            Dragging::Background {
                last_mouse_local_xy,
            } => {
                let delta = mouse_local_xy - *last_mouse_local_xy;
                self.start_at -= PixelSize::from(delta.x) * self.time_per_pixel;
                *last_mouse_local_xy = mouse_local_xy;
            }
            Dragging::Keyframe { ref action_ticket } => {
                self.animation_history
                    .update_action(*action_ticket, |action: &mut DraggingKeyframeAction| {
                        action.time_per_pixel = self.time_per_pixel;
                        action.start_at = self.start_at;
                        action.drag_end_x = mouse_local_xy.x.into();
                    })
                    .unwrap();
            }
        }
    }
    fn crate_new_keyframe(&self, mouse_local_xy: Xy<f32>) {
        if self.selected_layer_id.is_none() {
            return;
        }
        let selected_layer_id = self.selected_layer_id.as_ref().unwrap();

        struct CreateNewKeyframeAction {
            layer_id: String,
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

                    add_new_point(&mut layer.image.x, time, PixelSize::from(0.0));
                    add_new_point(&mut layer.image.y, time, PixelSize::from(0.0));
                    add_new_point(&mut layer.image.width, time, Percent::new(100.0));
                    add_new_point(&mut layer.image.height, time, Percent::new(100.0));
                    add_new_point(&mut layer.image.rotation_angle, time, Degree::from(0.0));
                    add_new_point(&mut layer.image.opacity, time, OneZero::from(1.0));
                    Ok(animation)
                } else {
                    Err("layer not found".into())
                }
            }
        }

        if let Some(action_ticket) =
            self.animation_history
                .try_set_action(CreateNewKeyframeAction {
                    layer_id: selected_layer_id.clone(),
                    time: self.start_at + PixelSize::from(mouse_local_xy.x) * self.time_per_pixel,
                })
        {
            self.animation_history.act(action_ticket).unwrap();
        }
    }
}

fn move_point(layer: &mut Layer, point_id: &str, to_time: Time) {
    move_point_in_graph(&mut layer.image.x, point_id, to_time);
    move_point_in_graph(&mut layer.image.y, point_id, to_time);
    move_point_in_graph(&mut layer.image.width, point_id, to_time);
    move_point_in_graph(&mut layer.image.height, point_id, to_time);
    move_point_in_graph(&mut layer.image.opacity, point_id, to_time);
    move_point_in_graph(&mut layer.image.rotation_angle, point_id, to_time);
}

fn move_point_in_graph<T: KeyframeValue + Clone>(
    graph: &mut KeyframeGraph<T>,
    point_id: &str,
    to_time: Time,
) {
    if let Some(point) = graph.get_point(point_id) {
        let mut point = point.clone();
        point.time = to_time;
        graph.put(point, animation::KeyframeLine::Linear);
    }
}

fn add_new_point<T: KeyframeValue + Clone>(
    graph: &mut KeyframeGraph<T>,
    time: Time,
    default_value: T,
) {
    let value_x = graph
        .get_value(time)
        .or_else(|| graph.get_last_point().map(|point| point.value.clone()))
        .unwrap_or(default_value);

    graph.put(
        KeyframePoint::new(time, value_x),
        animation::KeyframeLine::Linear,
    );
}

fn delete_point(layer: &mut Layer, point_id: &str) {
    layer.image.x.delete(point_id);
    layer.image.y.delete(point_id);
    layer.image.width.delete(point_id);
    layer.image.height.delete(point_id);
    layer.image.opacity.delete(point_id);
    layer.image.rotation_angle.delete(point_id);
}

struct DraggingKeyframeAction {
    layer_id: String,
    point_ids: Vec<String>,
    drag_end_x: PixelSize,
    anchor_x: PixelSize,
    start_at: Time,
    time_per_pixel: TimePerPixel,
}
impl Act<Animation> for DraggingKeyframeAction {
    fn act(&self, state: &Animation) -> Result<Animation, Box<dyn std::error::Error>> {
        let mut animation = state.clone();
        if let Some(layer) = animation
            .layers
            .iter_mut()
            .find(|layer| layer.id.eq(&self.layer_id))
        {
            let to_time = self.start_at + (self.drag_end_x - self.anchor_x) * self.time_per_pixel;

            for point_id in &self.point_ids {
                move_point(layer, &point_id, to_time)
            }

            Ok(animation)
        } else {
            Err("layer not found".into())
        }
    }
}
