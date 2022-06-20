use super::*;
use namui::animation::{Animate, Layer};

impl WysiwygWindow {
    pub(crate) fn render_layer(&self, layer: &Layer) -> namui::RenderingTree {
        layer
            .image
            .render(self.playback_time)
            .with_mouse_cursor(if self.selected_layer_id == Some(layer.id.clone()) {
                let is_dragging = false; // TODO
                if is_dragging {
                    namui::MouseCursor::Move
                } else {
                    namui::MouseCursor::Pointer
                }
            } else {
                MouseCursor::Pointer
            })
            .attach_event(|builder| {
                let layer_id = layer.id.clone();
                builder.on_mouse_down(move |_| {
                    namui::event::send(super::Event::LayerClicked {
                        layer_id: layer_id.clone(),
                    });
                })
            })
    }
}
