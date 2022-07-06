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
            self.render_lines(wh, &selected_layer),
            self.render_keyframes(wh, &selected_layer),
        ])
    }
}
