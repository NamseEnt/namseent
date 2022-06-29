use super::*;

enum PropertyContextMapping {
    X(PropertyContext<PixelSize>),
    Y(PropertyContext<PixelSize>),
    Width(PropertyContext<Percent>),
    Height(PropertyContext<Percent>),
    RotationAngle(PropertyContext<Degree>),
    Opacity(PropertyContext<OneZero>),
}
pub(super) struct MovePointByAction {
    point_address: PointAddress,
    delta_y: PixelSize,
    property_context: PropertyContextMapping,
}

impl Act<Animation> for MovePointByAction {
    fn act(&self, state: &Animation) -> Result<Animation, Box<dyn std::error::Error>> {
        let mut animation = state.clone();

        if let Some(layer) = animation
            .layers
            .iter_mut()
            .find(|layer| layer.id.eq(&self.point_address.layer_id))
        {
            match &self.property_context {
                PropertyContextMapping::X(property_context) => {
                    let mut point = layer
                        .image
                        .x
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = PixelSize::from_f32(
                        point.value.to_f32().unwrap()
                            + (property_context.value_per_pixel * self.delta_y)
                                .to_f32()
                                .unwrap(),
                    )
                    .unwrap();

                    layer.image.x.put(point, animation::KeyframeLine::Linear)
                }
                PropertyContextMapping::Y(property_context) => {
                    let mut point = layer
                        .image
                        .y
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = PixelSize::from_f32(
                        point.value.to_f32().unwrap()
                            + (property_context.value_per_pixel * self.delta_y)
                                .to_f32()
                                .unwrap(),
                    )
                    .unwrap();

                    layer.image.y.put(point, animation::KeyframeLine::Linear)
                }
                PropertyContextMapping::Width(property_context) => {
                    let mut point = layer
                        .image
                        .width_percent
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = Percent::from_f32(
                        point.value.to_f32().unwrap()
                            + (property_context.value_per_pixel * self.delta_y)
                                .to_f32()
                                .unwrap(),
                    )
                    .unwrap();

                    layer
                        .image
                        .width_percent
                        .put(point, animation::KeyframeLine::Linear)
                }
                PropertyContextMapping::Height(property_context) => {
                    let mut point = layer
                        .image
                        .height_percent
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = Percent::from_f32(
                        point.value.to_f32().unwrap()
                            + (property_context.value_per_pixel * self.delta_y)
                                .to_f32()
                                .unwrap(),
                    )
                    .unwrap();

                    layer
                        .image
                        .height_percent
                        .put(point, animation::KeyframeLine::Linear)
                }
                PropertyContextMapping::RotationAngle(property_context) => {
                    let mut point = layer
                        .image
                        .rotation_angle
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = Degree::from_f32(
                        point.value.to_f32().unwrap()
                            + (property_context.value_per_pixel * self.delta_y)
                                .to_f32()
                                .unwrap(),
                    )
                    .unwrap();

                    layer
                        .image
                        .rotation_angle
                        .put(point, animation::KeyframeLine::Linear)
                }
                PropertyContextMapping::Opacity(property_context) => {
                    let mut point = layer
                        .image
                        .opacity
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = OneZero::from_f32(
                        point.value.to_f32().unwrap()
                            + (property_context.value_per_pixel * self.delta_y)
                                .to_f32()
                                .unwrap(),
                    )
                    .unwrap();

                    layer
                        .image
                        .opacity
                        .put(point, animation::KeyframeLine::Linear)
                }
            };

            Ok(animation)
        } else {
            Err("layer not found".into())
        }
    }
}

impl GraphWindow {
    pub(super) fn get_move_by_action(
        &self,
        point_address: &PointAddress,
        delta_y: PixelSize,
    ) -> MovePointByAction {
        MovePointByAction {
            delta_y,
            point_address: point_address.clone(),
            property_context: match point_address.property_name {
                PropertyName::X => PropertyContextMapping::X(self.x_context.clone()),
                PropertyName::Y => PropertyContextMapping::Y(self.y_context.clone()),
                PropertyName::Width => PropertyContextMapping::Width(self.width_context.clone()),
                PropertyName::Height => PropertyContextMapping::Height(self.height_context.clone()),
                PropertyName::RotationAngle => {
                    PropertyContextMapping::RotationAngle(self.rotation_angle_context.clone())
                }
                PropertyName::Opacity => {
                    PropertyContextMapping::Opacity(self.opacity_context.clone())
                }
            },
        }
    }
}
