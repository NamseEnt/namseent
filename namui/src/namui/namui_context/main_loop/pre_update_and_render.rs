use crate::namui::render::DownUp;

use super::*;

impl NamuiContext {
    pub(super) fn pre_update_and_render(&mut self, event: &Event) {
        if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::AnimationFrame => {
                    invoke_and_flush_all_animation_frame_callbacks();
                }
                NamuiEvent::MouseDown(raw_mouse_event) => {
                    {
                        let managers = managers();
                        managers
                            .text_input_manager
                            .on_mouse_down(&self, &raw_mouse_event);
                    }
                    self.rendering_tree.call_mouse_event(
                        MouseEventType::Down,
                        raw_mouse_event,
                        &self,
                    );
                }
                NamuiEvent::MouseUp(raw_mouse_event) => {
                    {
                        let managers = managers();
                        managers
                            .text_input_manager
                            .on_mouse_up(&self, &raw_mouse_event);
                    }
                    self.rendering_tree.call_mouse_event(
                        MouseEventType::Up,
                        raw_mouse_event,
                        &self,
                    );
                }
                NamuiEvent::MouseMove(raw_mouse_event) => {
                    {
                        let managers = managers();
                        managers
                            .text_input_manager
                            .on_mouse_move(&self, &raw_mouse_event);
                    }
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
                _ => {}
            }
        }
    }
}
