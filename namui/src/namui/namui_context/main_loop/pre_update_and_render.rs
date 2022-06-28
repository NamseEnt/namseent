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
                    crate::system::text_input::on_mouse_down(&self, &raw_mouse_event);

                    self.rendering_tree.call_mouse_event(
                        MouseEventType::Down,
                        raw_mouse_event,
                        &self,
                    );
                }
                NamuiEvent::MouseUp(raw_mouse_event) => {
                    crate::system::text_input::on_mouse_up();

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

    fn resize_canvas(&mut self, screen_size: Wh<i16>) {
        if let Some(canvas_element) = get_canvas_element() {
            canvas_element.set_width(screen_size.width as u32);
            canvas_element.set_height(screen_size.height as u32);
            let canvas_kit_surface = make_canvas_surface(canvas_element);
            let surface = Surface::new(canvas_kit_surface);
            self.surface = surface;
        };
    }
}

fn get_canvas_element() -> Option<HtmlCanvasElement> {
    window()
        .document()
        .and_then(|document| document.get_element_by_id("canvas"))
        .and_then(|element| match element.dyn_into::<HtmlCanvasElement>() {
            Ok(canvas) => Some(canvas),
            Err(_) => None,
        })
}

fn make_canvas_surface(canvas_element: HtmlCanvasElement) -> CanvasKitSurface {
    CANVAS_KIT
        .get()
        .unwrap()
        .MakeCanvasSurface(&canvas_element)
        .unwrap()
}
