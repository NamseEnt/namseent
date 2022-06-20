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
        fn for_f32_based<TValue: KeyframeValue + Copy + From<f32> + Into<f32>>(
            property_context: &mut PropertyContext<TValue>,
            delta: f32,
            anchor_y: f32,
            row_height: f32,
        ) {
            let bottom_to_anchor = PixelSize::from(row_height - anchor_y);

            let next_value_per_pixel = property_context
                .zoom
                .zoom(property_context.value_per_pixel, delta.into());

            let zoomed_ratio = next_value_per_pixel / property_context.value_per_pixel;

            let zero_to_anchor_y = property_context.pixel_size_zero_to_bottom + bottom_to_anchor;
            let zoomed_zero_to_anchor_y = zero_to_anchor_y / zoomed_ratio;

            let next_pixel_size_zero_to_bottom = zoomed_zero_to_anchor_y - bottom_to_anchor;

            property_context.value_per_pixel = next_value_per_pixel;
            property_context.pixel_size_zero_to_bottom = next_pixel_size_zero_to_bottom;
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
        fn for_f32_based<TValue: KeyframeValue + Copy + From<f32> + Into<f32>>(
            property_context: &mut PropertyContext<TValue>,
            delta: f32,
        ) {
            property_context.pixel_size_zero_to_bottom += PixelSize::from(delta);
        }

        match property_name {
            PropertyName::X => for_f32_based(&mut self.x_context, delta),
            PropertyName::Y => for_f32_based(&mut self.y_context, delta),
            PropertyName::Width => for_f32_based(&mut self.width_context, delta),
            PropertyName::Height => for_f32_based(&mut self.height_context, delta),
            PropertyName::RotationAngle => for_f32_based(&mut self.rotation_angle_context, delta),
            PropertyName::Opacity => for_f32_based(&mut self.opacity_context, delta),
        }
    }
}
