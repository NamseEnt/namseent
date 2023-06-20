use super::*;

impl NamuiContext {
    pub(super) fn pre_update_and_render(&mut self, event: &Event) {
        event.is::<NamuiEvent>(|event| match event {
            NamuiEvent::AnimationFrame => {
                invoke_and_flush_all_animation_frame_callbacks();
                if let Some(screen_size) = self.is_surface_resize_requested.take() {
                    crate::graphics::resize_surface(screen_size);
                }
            }
            NamuiEvent::ScreenResize(screen_size) => {
                self.is_surface_resize_requested = Some(screen_size.clone());
            }
            NamuiEvent::DeepLinkOpened(_) | NamuiEvent::FileDrop(_) => {}
        });
    }
}
