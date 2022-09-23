use super::*;
use namui::animation::{Animate, Layer};
mod editing_tool;
mod moving_path;

impl WysiwygWindow {
    pub(super) fn render_layer(&self, props: &Props, layer: &Layer) -> namui::RenderingTree {
        let rendered_image = layer.image.render(props.playback_time);
        let mut rendering_tree = vec![rendered_image];

        let is_selected_layer = props.selected_layer_id == Some(layer.id.clone());

        if is_selected_layer {
            rendering_tree.push(self.render_moving_path(layer));

            let is_playback_time_on_editing_target_keyframe = match &props.editing_target {
                Some(EditingTarget::Keyframe { point_id, layer_id }) => {
                    layer.id.eq(layer_id)
                        && if let Some(point) =
                            layer.image.image_keyframe_graph.get_point(*point_id)
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
            let keyframe = layer
                .image
                .image_keyframe_graph
                .get_value(props.playback_time)?;
            let keyframe = keyframe;
            let x = keyframe.x();
            let y = keyframe.y();
            let wh = layer.image.get_image_px_wh(props.playback_time)?;
            let anchor_xy = layer.image.get_anchor_px_wh(props.playback_time)?;

            let rotation_angle = keyframe.rotation_angle();

            Some(translate(
                x,
                y,
                rotate(
                    rotation_angle,
                    translate(
                        -anchor_xy.x,
                        -anchor_xy.y,
                        simple_rect(
                            wh,
                            Color::grayscale_f01(0.5),
                            px(self.real_px_per_screen_px),
                            Color::TRANSPARENT,
                        ),
                    ),
                ),
            ))
        })
    }
}
