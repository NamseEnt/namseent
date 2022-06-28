use std::time::Duration;

use super::*;

impl NamuiContext {
    pub(super) fn post_update_and_render(&mut self, event: &Event) {
        if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::AnimationFrame => {
                    self.update_fps_info();

                    self.rendering_tree.draw(&self);

                    self.set_mouse_cursor();

                    crate::system::graphics::surface().flush();

                    if self.fps_info.frame_count == 0 {
                        crate::log!("event_count: {}", self.event_count);
                        self.event_count = 0;
                    }
                }
                _ => {}
            }
        }
        let now = crate::now();
        while let Some(timeout) = pull_timeout(now) {
            timeout();
        }
    }

    fn update_fps_info(&mut self) {
        let now = crate::now();
        let duration = now - self.fps_info.last_60_frame_time;

        if duration > Duration::from_secs(1) {
            self.fps_info.last_60_frame_time = crate::now();
            self.fps_info.fps = (self.fps_info.frame_count as f32 / duration.as_secs_f32()) as u16;
            self.fps_info.frame_count = 0;

            crate::log!("FPS: {}", self.fps_info.fps);
        } else {
            self.fps_info.frame_count += 1;
        }
    }

    fn set_mouse_cursor(&self) {
        let mouse_xy = {
            let mouse_position = crate::system::mouse::mouse_position();
            Xy {
                x: mouse_position.x as f32,
                y: mouse_position.y as f32,
            }
        };

        let cursor = self
            .rendering_tree
            .get_mouse_cursor(mouse_xy)
            .unwrap_or(MouseCursor::Default);

        crate::system::mouse::set_mouse_cursor(&cursor);

        if let MouseCursor::Custom(custom) = cursor {
            absolute(mouse_xy.x, mouse_xy.y, custom).draw(self);
        }
    }
}
