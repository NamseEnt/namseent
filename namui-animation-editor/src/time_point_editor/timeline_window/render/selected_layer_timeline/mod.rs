mod keyframes;
mod lines;

use super::*;

impl TimelineWindow {
    pub(super) fn render_selected_layer_timeline(
        &self,
        wh: Wh<Px>,
        props: &Props,
    ) -> RenderingTree {
        let selected_layer = props
            .selected_layer_id
            .as_ref()
            .and_then(|layer_id| props.layers.iter().find(|layer| layer.id.eq(layer_id)));

        if selected_layer.is_none() {
            return RenderingTree::Empty;
        }
        let selected_layer = selected_layer.unwrap();

        render([
            self.render_lines(props, wh, &selected_layer),
            self.render_keyframes(props, wh, &selected_layer),
        ])
        .attach_event(|builder| {
            let window_id = self.window_id.clone();
            let window_wh = props.wh;
            builder.on_mouse_down_out(move |event| {
                if event.button == Some(MouseButton::Left) {
                    let window_global_xy = event
                        .namui_context
                        .get_rendering_tree_xy_by_id(&window_id)
                        .unwrap();

                    let window_rect = Rect::from_xy_wh(window_global_xy, window_wh);

                    if window_rect.is_xy_inside(event.global_xy) {
                        namui::event::send(Event::MouseLeftDownOutOfEditingTargetButInWindow);
                    }
                }
            });
        })
    }
}
