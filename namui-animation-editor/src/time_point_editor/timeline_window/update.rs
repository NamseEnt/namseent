use super::*;
use crate::zoom::zoom_time_per_pixel;

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
        }
    }
}
