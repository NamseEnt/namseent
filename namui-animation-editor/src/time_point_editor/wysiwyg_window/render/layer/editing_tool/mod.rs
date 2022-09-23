use super::*;
use namui::types::Px;
mod moving;
mod resize;
mod rotate;

impl WysiwygWindow {
    pub(super) fn render_editing_tool(&self, props: &Props, layer: &Layer) -> namui::RenderingTree {
        let keyframe_point = layer
            .image
            .image_keyframe_graph
            .get_point_by_time(props.playback_time)
            .unwrap();
        let x = keyframe_point.value.x();
        let y = keyframe_point.value.y();
        let wh = layer.image.get_image_px_wh(props.playback_time).unwrap();
        let anchor_xy = layer.image.get_anchor_px_wh(props.playback_time).unwrap();

        let rotation_angle = keyframe_point.value.rotation_angle();

        let image_anchor_local_xy = Xy { x, y } - self.real_left_top_xy;

        translate(
            x,
            y,
            rotate(
                rotation_angle,
                translate(
                    -anchor_xy.x,
                    -anchor_xy.y,
                    render([
                        self.render_border_with_move_handling(wh, keyframe_point.id(), layer.id),
                        self.render_resize_circles(
                            wh,
                            keyframe_point.id(),
                            layer.id,
                            rotation_angle,
                        ),
                        self.render_rotation_tool(
                            wh,
                            keyframe_point.id(),
                            layer.id.clone(),
                            image_anchor_local_xy,
                        ),
                    ]),
                ),
            ),
        )
    }
}
