use super::*;
use namui::types::Px;
mod moving;
mod resize;
mod rotate;

impl WysiwygWindow {
    pub(super) fn render_editing_tool(
        &self,
        layer: &Layer,
        playback_time: Time,
    ) -> namui::RenderingTree {
        let x = layer.image.x.get_value(playback_time).unwrap();
        let y = layer.image.y.get_value(playback_time).unwrap();
        let wh = layer.image.get_image_px_wh(playback_time).unwrap();
        let anchor_xy = layer.image.get_anchor_px_wh(playback_time).unwrap();

        let rotation_angle = layer.image.rotation_angle.get_value(playback_time).unwrap();

        let image_anchor_local_xy = Xy {
            x: x.into(),
            y: y.into(),
        } - self.real_left_top_xy;

        translate(
            x.into(),
            y.into(),
            rotate(
                rotation_angle.as_radians(),
                translate(
                    (-anchor_xy.x).into(),
                    (-anchor_xy.y).into(),
                    render([
                        self.render_border_with_move_handling(wh, playback_time, &layer.id),
                        self.render_resize_circles(wh, playback_time, &layer.id, rotation_angle),
                        self.render_rotation_tool(
                            wh,
                            playback_time,
                            layer.id.clone(),
                            image_anchor_local_xy,
                        ),
                    ]),
                ),
            ),
        )
    }
}
