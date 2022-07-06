use super::*;

impl WysiwygWindow {
    pub(super) fn render_border_with_move_handling(
        &self,
        wh: Wh<Px>,
        playback_time: Time,
        layer_id: &str,
    ) -> RenderingTree {
        simple_rect(
            Wh {
                width: wh.width.into(),
                height: wh.height.into(),
            },
            Color::grayscale_f01(0.2),
            px(2.0) * self.real_px_per_screen_px,
            Color::TRANSPARENT,
        )
        .with_mouse_cursor({
            let is_dragging = matches!(self.dragging, Some(Dragging::ImageBody { .. }));
            if is_dragging {
                namui::MouseCursor::Move
            } else {
                namui::MouseCursor::Pointer
            }
        })
        .attach_event(|builder| {
            let layer_id = layer_id.to_string();
            let window_id = self.window_id.clone();

            builder.on_mouse_down_in(move |event| {
                let window_global_xy = event
                    .namui_context
                    .get_rendering_tree_xy_by_id(&window_id)
                    .unwrap();
                let anchor_xy = event.global_xy - window_global_xy;

                namui::event::send(super::Event::SelectedLayerMouseDown {
                    layer_id: layer_id.clone(),
                    anchor_xy,
                    playback_time,
                });
            });
        })
    }
}
