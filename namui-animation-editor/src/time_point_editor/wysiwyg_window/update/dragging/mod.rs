mod moving;
mod resize;
mod rotate;

use super::*;
pub(crate) use moving::*;
use namui::animation::*;
pub(crate) use resize::*;
pub(crate) use rotate::*;

impl WysiwygWindow {
    pub fn handle_dragging(&mut self, mouse_local_xy: Xy<Px>) {
        if self.dragging.is_none() {
            return;
        }
        let dragging = self.dragging.as_ref().unwrap();
        match dragging {
            Dragging::Background { anchor_xy } => {
                let delta = (mouse_local_xy - anchor_xy) * self.real_px_per_screen_px;
                self.real_left_top_xy = self.real_left_top_xy - delta;

                self.dragging = Some(Dragging::Background {
                    anchor_xy: mouse_local_xy,
                });
            }
            &Dragging::ResizeCircle { ticket } => {
                self.animation_history
                    .update_action(ticket, |action: &mut DragResizeCircleAction| {
                        action.real_px_per_screen_px = self.real_px_per_screen_px;
                        action.last_mouse_local_xy = mouse_local_xy;
                    })
                    .unwrap();
            }
            &Dragging::ImageBody { ticket } => {
                self.animation_history
                    .update_action(ticket, |action: &mut DragImageBodyAction| {
                        action.real_px_per_screen_px = self.real_px_per_screen_px;
                        action.last_mouse_local_xy = mouse_local_xy;
                    })
                    .unwrap();
            }
            &Dragging::Rotation { ticket } => {
                self.animation_history
                    .update_action(ticket, |action: &mut DragRotationAction| {
                        action.end_mouse_real_xy = mouse_local_xy * self.real_px_per_screen_px;
                    })
                    .unwrap();
            }
        }
    }
}
enum WidthHeight {
    Width,
    Height,
}

enum XY {
    X,
    Y,
}

fn update_xy(
    layer: &mut Layer,
    point_id: Uuid,
    delta: Px,
    x_y: XY,
) -> Result<(), Box<dyn std::error::Error>> {
    layer
        .image
        .image_keyframe_graph
        .update_point(point_id, |point| match x_y {
            XY::X => point.value.set_x(point.value.x() + delta),
            XY::Y => point.value.set_y(point.value.y() + delta),
        })
}
fn update_size(
    layer: &mut Layer,
    point_id: Uuid,
    delta: Px,
    width_height: WidthHeight,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_url = layer
        .image
        .image_source_url
        .clone()
        .ok_or(format!("layer {} has no image source url", layer.id))?;
    let image = namui::image::try_load_url(&image_url)
        .ok_or(format!("failed to load image {}", image_url))?;
    let image_wh = image.size();
    layer
        .image
        .image_keyframe_graph
        .update_point(point_id, |point| {
            let image_axis_length = match width_height {
                WidthHeight::Width => Px::from(image_wh.width),
                WidthHeight::Height => Px::from(image_wh.height),
            };

            let current_value = match width_height {
                WidthHeight::Width => point.value.width_percent(),
                WidthHeight::Height => point.value.height_percent(),
            };

            let next_value =
                Percent::from(((image_axis_length * current_value) + delta) / image_axis_length);

            match width_height {
                WidthHeight::Width => point.value.set_width_percent(next_value),
                WidthHeight::Height => point.value.set_height_percent(next_value),
            }
        })
}
