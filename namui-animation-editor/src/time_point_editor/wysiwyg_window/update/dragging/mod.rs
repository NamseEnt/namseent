use super::*;
use namui::{
    animation::{KeyframePoint, Layer},
    types::Percent,
};
mod resize;
pub(super) use resize::*;
mod rotate;
pub(super) use rotate::*;
mod moving;
pub(super) use moving::*;

impl WysiwygWindow {
    pub fn handle_dragging(&mut self, mouse_local_xy: Xy<Px>) {
        if self.dragging.is_none() {
            return;
        }
        let dragging = self.dragging.as_ref().unwrap();
        match dragging {
            Dragging::Background { anchor_xy } => {
                let delta = self.real_px_per_screen_px * (mouse_local_xy - anchor_xy);
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
                        action.end_mouse_real_xy = self.real_px_per_screen_px * mouse_local_xy;
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

fn update_xy(layer: &mut Layer, playback_time: Time, delta: Px, x_y: XY) {
    let graph = match x_y {
        XY::X => &mut layer.image.x,
        XY::Y => &mut layer.image.y,
    };
    let value = graph.get_value(playback_time).unwrap();
    let next = value + delta;
    graph.put(
        KeyframePoint::new(playback_time, next),
        animation::KeyframeLine::Linear,
    );
}
fn update_size(layer: &mut Layer, playback_time: Time, delta: Px, width_height: WidthHeight) {
    let image_url = layer.image.image_source_url.clone().unwrap();
    let image = namui::image::try_load(&image_url).unwrap();
    let image_wh = image.size();

    let graph = match width_height {
        WidthHeight::Width => &mut layer.image.width_percent,
        WidthHeight::Height => &mut layer.image.height_percent,
    };
    let value = graph.get_value(playback_time).unwrap();
    let image_value = match width_height {
        WidthHeight::Width => image_wh.width,
        WidthHeight::Height => image_wh.height,
    };
    let current = image_value * value;
    let next = current + delta;
    let next_value = Percent::from(next / image_value);
    graph.put(
        KeyframePoint::new(playback_time, next_value),
        animation::KeyframeLine::Linear,
    );
}
