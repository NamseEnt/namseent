use super::*;

pub(in super::super) struct DragImageBodyAction {
    pub layer_id: String,
    pub anchor_xy: Xy<f32>,
    pub last_mouse_local_xy: Xy<f32>,
    pub playback_time: Time,
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

            update_xy(layer, self.playback_time, delta_in_real.x, XY::X);
            update_xy(layer, self.playback_time, delta_in_real.y, XY::Y);

            Ok(animation)
        } else {
            Err("layer not found".into())
        }
    }
}
