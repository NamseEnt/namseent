use super::*;
use namui::animation::{KeyframePoint, KeyframeValue};

impl GraphWindow {
    pub(super) fn move_point_into_xy(
        &self,
        layer: &mut Layer,
        property_name: PropertyName,
        point_id: impl AsRef<str>,
        local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    ) {
        fn for_f32_based<TValue: KeyframeValue + Copy + From<f32> + Into<f32>>(
            context: &GraphWindowContext,
            graph: &mut KeyframeGraph<TValue>,
            property_context: &PropertyContext<TValue>,
            point_id: impl AsRef<str>,
            local_xy: Xy<f32>,
            row_wh: Wh<f32>,
        ) {
            let mut point = graph.get_point(point_id.as_ref()).unwrap().clone();

            let time_on_x = context.start_at + PixelSize::from(local_xy.x) * context.time_per_pixel;
            let value_on_y =
                property_context.get_value_on_y(row_wh.height.into(), local_xy.y.into());

            point.time = time_on_x;
            point.value = value_on_y;

            graph.put(point, animation::KeyframeLine::Linear);
        }

        match property_name {
            PropertyName::X => for_f32_based(
                &self.context,
                &mut layer.image.x,
                &self.x_context,
                point_id,
                local_xy,
                row_wh,
            ),
            PropertyName::Y => for_f32_based(
                &self.context,
                &mut layer.image.y,
                &self.y_context,
                point_id,
                local_xy,
                row_wh,
            ),
            PropertyName::Width => for_f32_based(
                &self.context,
                &mut layer.image.width,
                &self.width_context,
                point_id,
                local_xy,
                row_wh,
            ),
            PropertyName::Height => for_f32_based(
                &self.context,
                &mut layer.image.height,
                &self.height_context,
                point_id,
                local_xy,
                row_wh,
            ),
            PropertyName::RotationAngle => for_f32_based(
                &self.context,
                &mut layer.image.rotation_angle,
                &self.rotation_angle_context,
                point_id,
                local_xy,
                row_wh,
            ),
            PropertyName::Opacity => for_f32_based(
                &self.context,
                &mut layer.image.opacity,
                &self.opacity_context,
                point_id,
                local_xy,
                row_wh,
            ),
        }
    }
    pub(super) fn move_point_by_xy(
        &self,
        layer: &mut Layer,
        property_name: PropertyName,
        point_id: impl AsRef<str>,
        delta_xy: Xy<f32>,
    ) {
        fn for_f32_based<TValue: KeyframeValue + Copy + From<f32> + Into<f32>>(
            context: &GraphWindowContext,
            graph: &mut KeyframeGraph<TValue>,
            property_context: &PropertyContext<TValue>,
            point_id: impl AsRef<str>,
            delta_xy: Xy<f32>,
        ) {
            let mut point = graph.get_point(point_id.as_ref()).unwrap().clone();

            point.time += PixelSize::from(delta_xy.x) * context.time_per_pixel;
            point.value = (Into::<f32>::into(point.value)
                + (property_context.value_per_pixel * PixelSize::from(delta_xy.y)).into())
            .into();

            graph.put(point, animation::KeyframeLine::Linear);
        }

        match property_name {
            PropertyName::X => for_f32_based(
                &self.context,
                &mut layer.image.x,
                &self.x_context,
                point_id,
                delta_xy,
            ),
            PropertyName::Y => for_f32_based(
                &self.context,
                &mut layer.image.y,
                &self.y_context,
                point_id,
                delta_xy,
            ),
            PropertyName::Width => for_f32_based(
                &self.context,
                &mut layer.image.width,
                &self.width_context,
                point_id,
                delta_xy,
            ),
            PropertyName::Height => for_f32_based(
                &self.context,
                &mut layer.image.height,
                &self.height_context,
                point_id,
                delta_xy,
            ),
            PropertyName::RotationAngle => for_f32_based(
                &self.context,
                &mut layer.image.rotation_angle,
                &self.rotation_angle_context,
                point_id,
                delta_xy,
            ),
            PropertyName::Opacity => for_f32_based(
                &self.context,
                &mut layer.image.opacity,
                &self.opacity_context,
                point_id,
                delta_xy,
            ),
        }
    }
    pub(super) fn add_point_into_xy(
        &self,
        layer: &mut Layer,
        property_name: PropertyName,
        local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    ) {
        fn for_f32_based<TValue: KeyframeValue + Copy + From<f32> + Into<f32>>(
            context: &GraphWindowContext,
            graph: &mut KeyframeGraph<TValue>,
            property_context: &PropertyContext<TValue>,
            local_xy: Xy<f32>,
            row_wh: Wh<f32>,
        ) {
            let time_on_x = context.start_at + PixelSize::from(local_xy.x) * context.time_per_pixel;
            let value_on_y =
                property_context.get_value_on_y(row_wh.height.into(), local_xy.y.into());

            graph.put(
                KeyframePoint::new(time_on_x, value_on_y),
                animation::KeyframeLine::Linear,
            );
        }

        match property_name {
            PropertyName::X => for_f32_based(
                &self.context,
                &mut layer.image.x,
                &self.x_context,
                local_xy,
                row_wh,
            ),
            PropertyName::Y => for_f32_based(
                &self.context,
                &mut layer.image.y,
                &self.y_context,
                local_xy,
                row_wh,
            ),
            PropertyName::Width => for_f32_based(
                &self.context,
                &mut layer.image.width,
                &self.width_context,
                local_xy,
                row_wh,
            ),
            PropertyName::Height => for_f32_based(
                &self.context,
                &mut layer.image.height,
                &self.height_context,
                local_xy,
                row_wh,
            ),
            PropertyName::RotationAngle => for_f32_based(
                &self.context,
                &mut layer.image.rotation_angle,
                &self.rotation_angle_context,
                local_xy,
                row_wh,
            ),
            PropertyName::Opacity => for_f32_based(
                &self.context,
                &mut layer.image.opacity,
                &self.opacity_context,
                local_xy,
                row_wh,
            ),
        }
    }
}
