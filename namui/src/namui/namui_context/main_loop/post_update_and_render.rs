use super::*;

impl NamuiContext {
    pub(super) fn post_update_and_render(&mut self, event: &Event) {
        system::text_input::post_render(&self.rendering_tree);
        system::render::post_render(&self.rendering_tree);

        event.is::<NamuiEvent>(|event| match event {
            NamuiEvent::AnimationFrame => {
                self.update_fps_info();

                crate::graphics::surface()
                    .canvas()
                    .clear(Color::TRANSPARENT);

                self.rendering_tree.draw();

                self.set_mouse_cursor();

                crate::graphics::surface().flush();

                if self.fps_info.frame_count == 0 {
                    crate::log!("event_count: {}", self.event_count);
                    self.event_count = 0;
                }
            }
            NamuiEvent::FileDrop(raw_file_drop_event) => {
                self.rendering_tree
                    .call_file_drop_event(raw_file_drop_event);
            }
            NamuiEvent::ScreenResize(_)
            | NamuiEvent::DeepLinkOpened(_)
            | NamuiEvent::NoUpdateJustRender => {}
        });
        let now = crate::now();
        while let Some(timeout) = pull_timeout(now) {
            timeout();
        }
    }

    fn update_fps_info(&mut self) {
        let now = crate::now();
        let duration = now - self.fps_info.last_60_frame_time;

        if duration > Time::Sec(1.0) {
            self.fps_info.last_60_frame_time = crate::now();
            self.fps_info.fps = (self.fps_info.frame_count as f32 / duration.as_seconds()) as u16;
            self.fps_info.frame_count = 0;

            crate::log!("FPS: {}", self.fps_info.fps);
        } else {
            self.fps_info.frame_count += 1;
        }
    }

    fn set_mouse_cursor(&self) {
        let mouse_position = crate::mouse::position();
        let cursor = self
            .rendering_tree
            .get_mouse_cursor(mouse_position)
            .unwrap_or(MouseCursor::Default);

        crate::mouse::set_mouse_cursor(&cursor);

        if let MouseCursor::Custom(custom) = cursor {
            absolute(mouse_position.x, mouse_position.y, custom).draw();
        }
    }
}
