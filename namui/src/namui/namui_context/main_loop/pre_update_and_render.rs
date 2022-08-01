use super::*;

impl NamuiContext {
    pub(super) fn pre_update_and_render(&mut self, event: &Event) {
        if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::AnimationFrame => {
                    invoke_and_flush_all_animation_frame_callbacks();
                    if let Some(screen_size) = self.is_surface_resize_requested.take() {
                        crate::graphics::resize_surface(screen_size);
                    }
                }
                NamuiEvent::ScreenResize(screen_size) => {
                    self.is_surface_resize_requested = Some(screen_size.clone());
                }
                NamuiEvent::MouseDown(_)
                | NamuiEvent::MouseUp(_)
                | NamuiEvent::MouseMove(_)
                | NamuiEvent::KeyDown(_)
                | NamuiEvent::KeyUp(_)
                | NamuiEvent::Wheel(_)
                | NamuiEvent::DeepLinkOpened(_) => {}
            }
        }
    }
}
