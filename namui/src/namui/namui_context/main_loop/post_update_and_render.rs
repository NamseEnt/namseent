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

                    self.surface.flush();

                    if self.fps_info.frame_count == 0 {
                        log(format!("event_count: {}", self.event_count));
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
        let now = Namui::now();
        let duration = now - self.fps_info.last_60_frame_time;

        if duration > Duration::from_secs(1) {
            self.fps_info.last_60_frame_time = Namui::now();
            self.fps_info.fps = (self.fps_info.frame_count as f32 / duration.as_secs_f32()) as u16;
            self.fps_info.frame_count = 0;

            Namui::log(format!("FPS: {}", self.fps_info.fps));
        } else {
            self.fps_info.frame_count += 1;
        }
    }

    fn set_mouse_cursor(&self) {
        let managers = managers();
        let mouse_manager = &managers.mouse_manager;
        let mouse_xy = mouse_manager.mouse_position();

        let cursor = self
            .rendering_tree
            .get_mouse_cursor(Xy {
                x: mouse_xy.x as f32,
                y: mouse_xy.y as f32,
            })
            .unwrap_or(MouseCursor::Default);

        mouse_manager.set_mouse_cursor(cursor);
    }
}
