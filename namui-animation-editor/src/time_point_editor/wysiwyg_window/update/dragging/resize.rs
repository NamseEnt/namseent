use super::*;

pub(crate) struct DragResizeCircleAction {
    pub layer_id: String,
    pub anchor_xy: Xy<f32>,
    pub last_mouse_local_xy: Xy<f32>,
    pub playback_time: Time,
    pub real_pixel_size_per_screen_pixel_size: f32,
    pub location: ResizeCircleLocation,
    pub rotation_radian: Radian,
}
impl Act<Animation> for DragResizeCircleAction {
    fn act(&self, state: &Animation) -> Result<Animation, Box<dyn std::error::Error>> {
        let mut animation = state.clone();
        if let Some(layer) = animation
            .layers
            .iter_mut()
            .find(|layer| layer.id.eq(&self.layer_id))
        {
            let delta_in_real = self.real_pixel_size_per_screen_pixel_size
                * (self.last_mouse_local_xy - self.anchor_xy);
            let reversed_rotated_delta_in_real = Xy {
                x: delta_in_real.x * (-self.rotation_radian).cos()
                    - delta_in_real.y * (-self.rotation_radian).sin(),
                y: delta_in_real.x * (-self.rotation_radian).sin()
                    + delta_in_real.y * (-self.rotation_radian).cos(),
            };

            let (x, y, width, height) = (
                match self.location {
                    ResizeCircleLocation::LeftTop
                    | ResizeCircleLocation::RightTop
                    | ResizeCircleLocation::Left
                    | ResizeCircleLocation::Right
                    | ResizeCircleLocation::LeftBottom
                    | ResizeCircleLocation::RightBottom => Some(()),
                    _ => None,
                },
                match self.location {
                    ResizeCircleLocation::LeftTop
                    | ResizeCircleLocation::Top
                    | ResizeCircleLocation::RightTop
                    | ResizeCircleLocation::LeftBottom
                    | ResizeCircleLocation::Bottom
                    | ResizeCircleLocation::RightBottom => Some(()),
                    _ => None,
                },
                match self.location {
                    ResizeCircleLocation::LeftTop
                    | ResizeCircleLocation::Left
                    | ResizeCircleLocation::LeftBottom => Some(-1.0),
                    ResizeCircleLocation::RightTop
                    | ResizeCircleLocation::Right
                    | ResizeCircleLocation::RightBottom => Some(1.0),
                    _ => None,
                },
                match self.location {
                    ResizeCircleLocation::LeftTop
                    | ResizeCircleLocation::Top
                    | ResizeCircleLocation::RightTop => Some(-1.0),
                    ResizeCircleLocation::LeftBottom
                    | ResizeCircleLocation::Bottom
                    | ResizeCircleLocation::RightBottom => Some(1.0),
                    _ => None,
                },
            );

            if x.is_some() {
                update_xy(
                    layer,
                    self.playback_time,
                    self.rotation_radian.cos() * reversed_rotated_delta_in_real.x / 2.0,
                    XY::X,
                );
                update_xy(
                    layer,
                    self.playback_time,
                    self.rotation_radian.sin() * reversed_rotated_delta_in_real.x / 2.0,
                    XY::Y,
                );
            }
            if y.is_some() {
                update_xy(
                    layer,
                    self.playback_time,
                    -self.rotation_radian.sin() * reversed_rotated_delta_in_real.y / 2.0,
                    XY::X,
                );
                update_xy(
                    layer,
                    self.playback_time,
                    self.rotation_radian.cos() * reversed_rotated_delta_in_real.y / 2.0,
                    XY::Y,
                );
            }
            if let Some(ratio) = width {
                update_size(
                    layer,
                    self.playback_time,
                    reversed_rotated_delta_in_real.x * ratio,
                    WidthHeight::Width,
                );
            }
            if let Some(ratio) = height {
                update_size(
                    layer,
                    self.playback_time,
                    reversed_rotated_delta_in_real.y * ratio,
                    WidthHeight::Height,
                );
            }

            Ok(animation)
        } else {
            Err("layer not found".into())
        }
    }
}
