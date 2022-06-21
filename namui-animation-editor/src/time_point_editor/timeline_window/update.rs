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
            }
        }
    }
}
