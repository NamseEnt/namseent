use std::sync::Arc;

use namui::{animation::KeyframePoint, types::Percent};

use super::*;

impl WysiwygWindow {
    pub fn handle_dragging(&mut self, mouse_local_xy: Xy<f32>) {
        if self.dragging.is_none() {
            return;
        }
        let dragging = self.dragging.as_ref().unwrap();
        match dragging {
            Dragging::Background { anchor_xy } => {
                let delta =
                    self.real_pixel_size_per_screen_pixel_size * (mouse_local_xy - anchor_xy);
                self.real_left_top_xy = self.real_left_top_xy - delta;

                self.dragging = Some(Dragging::Background {
                    anchor_xy: mouse_local_xy,
                });
            }
            &Dragging::ResizeCircle {
                location,
                anchor_xy,
            } => {
                let delta_in_real =
                    self.real_pixel_size_per_screen_pixel_size * (mouse_local_xy - anchor_xy);

                let layer_id = self.selected_layer_id.clone().unwrap();

                let animation = self.animation.read();
                let layer = animation.layers.iter().find(|layer| layer.id.eq(&layer_id));
                if layer.is_none() {
                    return;
                }
                let mut layer = layer.unwrap().clone();

                let image_url = layer.image.image_source_url.clone().unwrap();
                let managers = namui::managers();
                let image = managers.image_manager.try_load(&image_url).unwrap();
                let image_wh = image.size();

                match location {
                    ResizeCircleLocation::LeftTop => todo!(),
                    ResizeCircleLocation::Top => todo!(),
                    ResizeCircleLocation::RightTop => todo!(),
                    ResizeCircleLocation::Left => todo!(),
                    ResizeCircleLocation::Right => {
                        let value = layer.image.width.get_value(self.playback_time).unwrap();
                        let current_width: f32 = (value * image_wh.width).into();
                        let next_width = current_width + delta_in_real.x;
                        let next_value = Percent::from(next_width / image_wh.width);
                        layer.image.width.put(
                            KeyframePoint::new(self.playback_time, next_value),
                            animation::KeyframeLine::Linear,
                        );
                    }
                    ResizeCircleLocation::LeftBottom => todo!(),
                    ResizeCircleLocation::Bottom => todo!(),
                    ResizeCircleLocation::RightBottom => todo!(),
                }

                namui::event::send(crate::Event::UpdateLayer(Arc::new(layer)));

                self.dragging = Some(Dragging::ResizeCircle {
                    location,
                    anchor_xy: mouse_local_xy,
                });
            }
        }
    }
}
