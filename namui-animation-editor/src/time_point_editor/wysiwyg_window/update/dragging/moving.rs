use super::*;

pub struct DragImageBodyAction {
    pub layer_id: namui::Uuid,
    pub anchor_xy: Xy<Px>,
    pub last_mouse_local_xy: Xy<Px>,
    pub keyframe_point_id: namui::Uuid,
    pub real_px_per_screen_px: f32,
}
impl Act<Animation> for DragImageBodyAction {
    fn act(&self, state: &Animation) -> Result<Animation, Box<dyn std::error::Error>> {
        let mut animation = state.clone();
        if let Some(layer) = animation
            .layers
            .iter_mut()
            .find(|layer| layer.id.eq(&self.layer_id))
        {
            let delta_in_real =
                self.real_px_per_screen_px * (self.last_mouse_local_xy - self.anchor_xy);

            update_xy(layer, self.keyframe_point_id, delta_in_real.x, XY::X)?;
            update_xy(layer, self.keyframe_point_id, delta_in_real.y, XY::Y)?;

            Ok(animation)
        } else {
            Err("layer not found".into())
        }
    }
}
