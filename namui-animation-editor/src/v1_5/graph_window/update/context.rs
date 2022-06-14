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
            let bottom_to_anchor = PixelSize(row_height - anchor_y);

            let next_value_per_pixel =
                zoom_f32_based_per_pixel(property_context.value_per_pixel, delta.into());

            let value_at_mouse_position: f32 = property_context.value_at_bottom.into()
                + (property_context.value_per_pixel * bottom_to_anchor).into();

            let next_value_at_bottom =
                value_at_mouse_position - (next_value_per_pixel * bottom_to_anchor).into();

            property_context.value_per_pixel = next_value_per_pixel;
            property_context.value_at_bottom = next_value_at_bottom.into();
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
        }
    }

    pub(super) fn move_property_context_by(&mut self, property_name: PropertyName, delta: f32) {
        fn for_f32_based<TValue: KeyframeValue + Copy + From<f32> + Into<f32>>(
            property_context: &mut PropertyContext<TValue>,
            delta: f32,
        ) {
            property_context.value_at_bottom =
                (Into::<f32>::into(property_context.value_at_bottom)
                    + (property_context.value_per_pixel * PixelSize(delta)).into())
                .into();
        }

        match property_name {
            PropertyName::X => for_f32_based(&mut self.x_context, delta),
            PropertyName::Y => for_f32_based(&mut self.y_context, delta),
            PropertyName::Width => for_f32_based(&mut self.width_context, delta),
            PropertyName::Height => for_f32_based(&mut self.height_context, delta),
            PropertyName::RotationAngle => for_f32_based(&mut self.rotation_angle_context, delta),
        }
    }
}

fn zoom_f32_based_per_pixel<TValue: KeyframeValue + Copy + From<f32> + Into<f32>>(
    target: ValuePerPixel<TValue>,
    delta: f32,
) -> ValuePerPixel<TValue> {
    const STEP: f32 = 400.0;
    const MIN: f32 = 1.0;
    const MAX: f32 = 100.0;

    let wheel = STEP * (target.value.into() / f32::from(target.pixel_size) / 10.0).log2();

    let next_wheel = wheel + delta;

    let zoomed = namui::math::num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);

    ValuePerPixel {
        value: zoomed.into(),
        pixel_size: 1.0_f32.into(),
    }
}
