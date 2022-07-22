use super::*;
use namui::animation::{Animate, Layer};
mod editing_tool;

impl WysiwygWindow {
    pub(super) fn render_layer(&self, props: &Props, layer: &Layer) -> namui::RenderingTree {
        let rendered_image = layer.image.render(props.playback_time);
        let mut rendering_tree = vec![rendered_image];

        let is_selected_layer = props.selected_layer_id == Some(layer.id.clone());

        if is_selected_layer {
            rendering_tree.push(layer.image.render_moving_path());

            let is_playback_time_on_editing_target_keyframe = match &props.editing_target {
                Some(EditingTarget::Keyframe { point_id, layer_id }) => {
                    layer.id.eq(layer_id)
                        && if let Some(point) = layer.image.image_keyframe_graph.get_point(point_id)
                        {
                            point.time == props.playback_time
                        } else {
                            false
                        }
                }
                _ => false,
            };

            let is_playback_time_in_visible_range = {
                match layer.image.get_visible_time_range() {
                    Some((start, end)) => {
                        start <= props.playback_time && props.playback_time <= end
                    }
                    None => false,
                }
            };

            let can_draw_editing_tool = is_playback_time_on_editing_target_keyframe;
            let can_draw_hint_bounding_box =
                !is_playback_time_on_editing_target_keyframe && is_playback_time_in_visible_range;

            if can_draw_editing_tool {
                rendering_tree.push(self.render_editing_tool(props, layer));
            } else if can_draw_hint_bounding_box {
                rendering_tree.push(self.render_hint_bounding_box(props, layer));
            }
        }

        render(rendering_tree)
    }

    fn render_hint_bounding_box(&self, props: &Props, layer: &animation::Layer) -> RenderingTree {
        try_render(|| {
            layer.image.try_render_bounding_box(
                props.playback_time,
                Color::grayscale_f01(0.5),
                px(self.real_px_per_screen_px),
                Color::TRANSPARENT,
            )
        })
    }
}
