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
        let mut rendering_tree = vec![layer.image.render(playback_time)];

        if is_selected_layer {
            rendering_tree.push(self.render_editing_tool(&layer, playback_time));
        }
        render(rendering_tree)
    }
}
