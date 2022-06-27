use super::*;
use namui::animation::{Animate, Layer};
mod editing_tool;
mod moving_path;

impl WysiwygWindow {
    pub(super) fn render_layer(
        &self,
        layer: &Layer,
        playback_time: Time,
        selected_layer_id: Option<String>,
    ) -> namui::RenderingTree {
        let rendered_image = layer.image.render(playback_time);
        let mut rendering_tree = vec![rendered_image];

        let is_selected_layer = selected_layer_id == Some(layer.id.clone());

        if is_selected_layer {
            rendering_tree.push(self.render_moving_path(layer));

            let is_playback_time_on_keyframe = layer
                .image
                .get_keyframe_infos()
                .iter()
                .any(|keyframe_info| keyframe_info.time == playback_time);
            let is_playback_time_in_visible_range = {
                match layer.image.get_visible_time_range() {
                    Some((start, end)) => start <= playback_time && playback_time <= end,
                    None => false,
                }
            };

            let can_draw_editing_tool = is_playback_time_on_keyframe;
            let can_draw_hint_bounding_box =
                !is_playback_time_on_keyframe && is_playback_time_in_visible_range;

            if can_draw_editing_tool {
                rendering_tree.push(self.render_editing_tool(layer, playback_time));
            } else if can_draw_hint_bounding_box {
                rendering_tree.push(self.render_hint_bounding_box(layer, playback_time));
            }
        }

        render(rendering_tree)
    }

    fn render_hint_bounding_box(
        &self,
        layer: &animation::Layer,
        playback_time: Time,
    ) -> RenderingTree {
        let x = layer.image.x.get_value(playback_time).unwrap();
        let y = layer.image.y.get_value(playback_time).unwrap();
        let wh = layer.image.get_image_pixel_size_wh(playback_time).unwrap();
        let anchor_xy = layer.image.get_anchor_pixel_size_wh(playback_time).unwrap();

        let rotation_radian = layer
            .image
            .rotation_angle
            .get_value(playback_time)
            .unwrap()
            .to_radian();

        translate(
            x.into(),
            y.into(),
            rotate(
                rotation_radian.into(),
                translate(
                    (-anchor_xy.x).into(),
                    (-anchor_xy.y).into(),
                    simple_rect(
                        Wh {
                            width: wh.width.into(),
                            height: wh.height.into(),
                        },
                        Color::grayscale_f01(0.5),
                        1.0 * self.real_pixel_size_per_screen_pixel_size,
                        Color::TRANSPARENT,
                    ),
                ),
            ),
        )
    }
}
