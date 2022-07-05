use super::*;

pub(super) enum PropertyContextMapping {
    X(PropertyContext<Px>),
    Y(PropertyContext<Px>),
    Width(PropertyContext<Percent>),
    Height(PropertyContext<Percent>),
    RotationAngle(PropertyContext<Angle>),
    Opacity(PropertyContext<OneZero>),
}
pub(super) struct MovePointToAction {
    pub(super) point_address: PointAddress,
    pub(super) row_height: Px,
    pub(super) y_in_row: Px,
    pub(super) property_context: PropertyContextMapping,
}

impl Act<Animation> for MovePointToAction {
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

                    point.value = property_context.get_value_on_y(self.row_height, self.y_in_row);
                    layer.image.x.put(point, animation::KeyframeLine::Linear);
                }
                PropertyContextMapping::Y(property_context) => {
                    let mut point = layer
                        .image
                        .y
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = property_context.get_value_on_y(self.row_height, self.y_in_row);
                    layer.image.y.put(point, animation::KeyframeLine::Linear);
                }
                PropertyContextMapping::Width(property_context) => {
                    let mut point = layer
                        .image
                        .width_percent
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = property_context.get_value_on_y(self.row_height, self.y_in_row);
                    layer
                        .image
                        .width_percent
                        .put(point, animation::KeyframeLine::Linear);
                }
                PropertyContextMapping::Height(property_context) => {
                    let mut point = layer
                        .image
                        .height_percent
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = property_context.get_value_on_y(self.row_height, self.y_in_row);
                    layer
                        .image
                        .height_percent
                        .put(point, animation::KeyframeLine::Linear);
                }
                PropertyContextMapping::RotationAngle(property_context) => {
                    let mut point = layer
                        .image
                        .rotation_angle
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = property_context.get_value_on_y(self.row_height, self.y_in_row);
                    layer
                        .image
                        .rotation_angle
                        .put(point, animation::KeyframeLine::Linear);
                }
                PropertyContextMapping::Opacity(property_context) => {
                    let mut point = layer
                        .image
                        .opacity
                        .get_point(&self.point_address.point_id)
                        .ok_or_else(|| "point not found")?
                        .clone();

                    point.value = property_context.get_value_on_y(self.row_height, self.y_in_row);
                    layer
                        .image
                        .opacity
                        .put(point, animation::KeyframeLine::Linear);
                }
            };

            Ok(animation)
        } else {
            Err("layer not found".into())
        }
    }
}

impl GraphWindow {
    pub(super) fn get_move_to_action(
        &self,
        point_address: &PointAddress,
        row_height: Px,
        y_in_row: Px,
    ) -> MovePointToAction {
        MovePointToAction {
            point_address: point_address.clone(),
            row_height,
            y_in_row,
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
