use super::*;
use namui::animation::{Animate, Layer};
mod editing_tool;

impl WysiwygWindow {
    pub(super) fn render_layer(
        &self,
        layer: &Layer,
        playback_time: Time,
        selected_layer_id: Option<String>,
    ) -> namui::RenderingTree {
        let is_selected_layer = selected_layer_id == Some(layer.id.clone());
        let mut rendered_image =
            layer
                .image
                .render(playback_time)
                .with_mouse_cursor(if is_selected_layer {
                    let is_dragging = matches!(self.dragging, Some(Dragging::ImageBody { .. }));
                    if is_dragging {
                        namui::MouseCursor::Move
                    } else {
                        namui::MouseCursor::Pointer
                    }
                } else {
                    MouseCursor::Default
                });

        if is_selected_layer {
            rendered_image = rendered_image.attach_event(|builder| {
                let layer_id = layer.id.clone();
                let real_left_top_xy = self.real_left_top_xy;
                let real_pixel_size_per_screen_pixel_size =
                    self.real_pixel_size_per_screen_pixel_size;

                builder.on_mouse_down(move |event| {
                    let real_anchor_xy = event.local_xy - real_left_top_xy;
                    let anchor_xy = Xy {
                        x: real_anchor_xy.x / real_pixel_size_per_screen_pixel_size,
                        y: real_anchor_xy.y / real_pixel_size_per_screen_pixel_size,
                    };
                    namui::event::send(super::Event::LayerClicked {
                        layer_id: layer_id.clone(),
                        anchor_xy,
                        playback_time,
                    });
                })
            });
        }

        let editing_tool =
            self.render_editing_tool(&layer, playback_time, &rendered_image, selected_layer_id);
        render([rendered_image, editing_tool])
    }
}
