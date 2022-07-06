use super::*;
use crate::namui::render::DownUp;

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
                NamuiEvent::MouseDown(raw_mouse_event) => {
                    crate::system::text_input::on_mouse_down_in(&self, &raw_mouse_event);

                    self.rendering_tree.call_mouse_event(
                        MouseEventType::Down,
                        raw_mouse_event,
                        &self,
                    );
                }
                NamuiEvent::MouseUp(raw_mouse_event) => {
                    crate::system::text_input::on_mouse_up_in();

                    self.rendering_tree.call_mouse_event(
                        MouseEventType::Up,
                        raw_mouse_event,
                        &self,
                    );
                }
                NamuiEvent::MouseMove(raw_mouse_event) => {
                    crate::system::text_input::on_mouse_move(&self, &raw_mouse_event);

                    self.rendering_tree.call_mouse_event(
                        MouseEventType::Move,
                        raw_mouse_event,
                        &self,
                    );
                }
                NamuiEvent::Wheel(raw_wheel_event) => {
                    self.rendering_tree.call_wheel_event(raw_wheel_event, &self);
                }
                NamuiEvent::KeyDown(raw_keyboard_event) => {
                    self.rendering_tree.call_keyboard_event(
                        raw_keyboard_event,
                        &self,
                        DownUp::Down,
                    );
                }
                NamuiEvent::KeyUp(raw_keyboard_event) => {
                    self.rendering_tree
                        .call_keyboard_event(raw_keyboard_event, &self, DownUp::Up);
                }
                NamuiEvent::ScreenResize(screen_size) => {
                    self.is_surface_resize_requested = Some(screen_size.clone());
                }
            }
        }
    }
}
