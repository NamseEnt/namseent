use super::*;

pub(crate) struct DragRotationAction {
    pub image_center_real_xy: Xy<Px>,
    pub start_mouse_real_xy: Xy<Px>,
    pub end_mouse_real_xy: Xy<Px>,
    pub playback_time: Time,
    pub layer_id: String,
}
impl Act<Animation> for DragRotationAction {
    fn act(&self, state: &Animation) -> Result<Animation, Box<dyn std::error::Error>> {
        let mut animation = state.clone();
        if let Some(layer) = animation
            .layers
            .iter_mut()
            .find(|layer| layer.id.eq(&self.layer_id))
        {
            let center_to_start_xy = self.start_mouse_real_xy - self.image_center_real_xy;
            let center_to_end_xy = self.end_mouse_real_xy - self.image_center_real_xy;
            let angle = center_to_start_xy.angle_to(center_to_end_xy);

            let previous_angle = layer
                .image
                .rotation_angle
                .get_value(self.playback_time)
                .unwrap();

            layer.image.rotation_angle.put(
                KeyframePoint::new(self.playback_time, previous_angle + angle),
                animation::KeyframeLine::Linear,
            );

            Ok(animation)
        } else {
            Err("layer not found".into())
        }
    }
}
