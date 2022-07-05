use super::*;
use namui::animation::KeyframeValue;

impl GraphWindow {
    pub(super) fn zoom_property_context(
        &mut self,
        property_name: PropertyName,
        delta: f32,
        anchor_y: f32,
        row_height: f32,
    ) {
        fn for_f32_based<TValue: KeyframeValue + Copy + FromPrimitive + ToPrimitive>(
            property_context: &mut PropertyContext<TValue>,
            delta: f32,
            anchor_y: f32,
            row_height: f32,
        ) {
            let bottom_to_anchor = Px::from(row_height - anchor_y);

            let next_value_per_px = property_context
                .zoom
                .zoom(property_context.value_per_px, delta.into());

            let zoomed_ratio = next_value_per_px / property_context.value_per_px;

            let zero_to_anchor_y = property_context.px_zero_to_bottom + bottom_to_anchor;
            let zoomed_zero_to_anchor_y = zero_to_anchor_y / zoomed_ratio;

            let next_px_zero_to_bottom = zoomed_zero_to_anchor_y - bottom_to_anchor;

            property_context.value_per_px = next_value_per_px;
            property_context.px_zero_to_bottom = next_px_zero_to_bottom;
        }

        match property_name {
            PropertyName::X => for_f32_based(&mut self.x_context, delta, anchor_y, row_height),
            PropertyName::Y => for_f32_based(&mut self.y_context, delta, anchor_y, row_height),
            PropertyName::Width => {
                for_f32_based(&mut self.width_context, delta, anchor_y, row_height)
            }
            PropertyName::Height => {
                for_f32_based(&mut self.height_context, delta, anchor_y, row_height)
            }
            PropertyName::RotationAngle => for_f32_based(
                &mut self.rotation_angle_context,
                delta,
                anchor_y,
                row_height,
            ),
            PropertyName::Opacity => {
                for_f32_based(&mut self.opacity_context, delta, anchor_y, row_height)
            }
        }
    }

    pub(super) fn move_property_context_by(&mut self, property_name: PropertyName, delta: f32) {
        fn for_f32_based<TValue: KeyframeValue + Copy + FromPrimitive + ToPrimitive>(
            property_context: &mut PropertyContext<TValue>,
            delta: f32,
        ) {
            property_context.px_zero_to_bottom += Px::from(delta);
        }

        match property_name {
            PropertyName::X => for_f32_based(&mut self.x_context, delta),
            PropertyName::Y => for_f32_based(&mut self.y_context, delta),
            PropertyName::Width => for_f32_based(&mut self.width_context, delta),
            PropertyName::Height => for_f32_based(&mut self.height_context, delta),
            PropertyName::RotationAngle => for_f32_based(&mut self.rotation_angle_context, delta),
            PropertyName::Opacity => for_f32_based(&mut self.opacity_context, delta),
        }

        match &self.dragging {
            Some(dragging) => match dragging {
                Dragging::Point { ticket, .. } => {
                    self.animation_history
                        .update_action(*ticket, |action: &mut MovePointToAction| {
                            action.property_context = match &action.property_context {
                                super::move_to::PropertyContextMapping::X(_) => {
                                    super::move_to::PropertyContextMapping::X(
                                        self.x_context.clone(),
                                    )
                                }
                                super::move_to::PropertyContextMapping::Y(_) => {
                                    super::move_to::PropertyContextMapping::Y(
                                        self.y_context.clone(),
                                    )
                                }
                                super::move_to::PropertyContextMapping::Width(_) => {
                                    super::move_to::PropertyContextMapping::Width(
                                        self.width_context.clone(),
                                    )
                                }
                                super::move_to::PropertyContextMapping::Height(_) => {
                                    super::move_to::PropertyContextMapping::Height(
                                        self.height_context.clone(),
                                    )
                                }
                                super::move_to::PropertyContextMapping::RotationAngle(_) => {
                                    super::move_to::PropertyContextMapping::RotationAngle(
                                        self.rotation_angle_context.clone(),
                                    )
                                }
                                super::move_to::PropertyContextMapping::Opacity(_) => {
                                    super::move_to::PropertyContextMapping::Opacity(
                                        self.opacity_context.clone(),
                                    )
                                }
                            }
                        })
                        .unwrap();
                }
                _ => {}
            },
            None => {}
        }
    }
}
