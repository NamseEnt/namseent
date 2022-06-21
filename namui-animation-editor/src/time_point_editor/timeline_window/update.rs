use super::*;
use crate::zoom::zoom_time_per_pixel;
use namui::animation::{KeyframeGraph, KeyframeValue};
use std::sync::Arc;

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
                &Event::TimelineClicked { mouse_local_xy } => {
                    if self.dragging.is_none() {
                        self.dragging = Some(Dragging::Background {
                            last_mouse_local_xy: mouse_local_xy,
                        });
                    }
                }
                &Event::TimelineMouseMoveIn { mouse_local_xy } => {
                    self.handle_timeline_dragging(mouse_local_xy);
                }
                Event::KeyframeClicked {
                    point_ids,
                    anchor_xy,
                } => {
                    if self.dragging.is_none() {
                        self.dragging = Some(Dragging::Keyframe {
                            point_ids: point_ids.clone(),
                            anchor_xy: *anchor_xy,
                        });
                    }
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseUp(_) => {
                    self.dragging = None;
                }
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
            Dragging::Keyframe {
                point_ids,
                anchor_xy,
            } => {
                if self.selected_layer_id.is_none() {
                    return;
                }
                let selected_layer_id = self.selected_layer_id.as_ref().unwrap();

                let animation = self.animation.read();

                let mut layer = animation
                    .layers
                    .iter()
                    .find(|layer| layer.id.eq(selected_layer_id))
                    .unwrap()
                    .clone();

                let to_time = self.start_at
                    + PixelSize::from(mouse_local_xy.x - anchor_xy.x) * self.time_per_pixel;

                for point_id in point_ids {
                    move_point(&mut layer, point_id, to_time)
                }

                namui::event::send(crate::Event::UpdateLayer(Arc::new(layer)));
            }
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
