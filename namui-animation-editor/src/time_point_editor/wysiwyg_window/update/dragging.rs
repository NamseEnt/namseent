use std::sync::Arc;

use namui::{
    animation::{KeyframePoint, Layer},
    types::Percent,
};

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
                playback_time,
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

                match location {
                    ResizeCircleLocation::LeftTop
                    | ResizeCircleLocation::Top
                    | ResizeCircleLocation::RightTop => {
                        self.update_size(
                            &mut layer,
                            playback_time,
                            delta_in_real,
                            WidthHeight::Height,
                            PlusMinus::Minus,
                        );
                        self.update_xy(
                            &mut layer,
                            playback_time,
                            delta_in_real,
                            XY::Y,
                            PlusMinus::Plus,
                        );
                    }
                    ResizeCircleLocation::LeftBottom
                    | ResizeCircleLocation::Bottom
                    | ResizeCircleLocation::RightBottom => {
                        self.update_size(
                            &mut layer,
                            playback_time,
                            delta_in_real,
                            WidthHeight::Height,
                            PlusMinus::Plus,
                        );
                    }
                    _ => {}
                }

                match location {
                    ResizeCircleLocation::LeftTop
                    | ResizeCircleLocation::Left
                    | ResizeCircleLocation::LeftBottom => {
                        self.update_size(
                            &mut layer,
                            playback_time,
                            delta_in_real,
                            WidthHeight::Width,
                            PlusMinus::Minus,
                        );
                        self.update_xy(
                            &mut layer,
                            playback_time,
                            delta_in_real,
                            XY::X,
                            PlusMinus::Plus,
                        );
                    }
                    ResizeCircleLocation::RightTop
                    | ResizeCircleLocation::Right
                    | ResizeCircleLocation::RightBottom => {
                        self.update_size(
                            &mut layer,
                            playback_time,
                            delta_in_real,
                            WidthHeight::Width,
                            PlusMinus::Plus,
                        );
                    }
                    _ => {}
                }

                namui::event::send(crate::Event::UpdateLayer(Arc::new(layer)));

                self.dragging = Some(Dragging::ResizeCircle {
                    location,
                    anchor_xy: mouse_local_xy,
                    playback_time,
                });
            }
            &Dragging::ImageBody {
                anchor_xy,
                playback_time,
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

                self.update_xy(
                    &mut layer,
                    playback_time,
                    delta_in_real,
                    XY::X,
                    PlusMinus::Plus,
                );
                self.update_xy(
                    &mut layer,
                    playback_time,
                    delta_in_real,
                    XY::Y,
                    PlusMinus::Plus,
                );

                namui::event::send(crate::Event::UpdateLayer(Arc::new(layer)));

                self.dragging = Some(Dragging::ImageBody {
                    anchor_xy: mouse_local_xy,
                    playback_time,
                });
            }
        }
    }
    fn update_size(
        &self,
        layer: &mut Layer,
        playback_time: Time,
        delta_in_real: Xy<f32>,
        width_height: WidthHeight,
        plus_minus: PlusMinus,
    ) {
        let image_url = layer.image.image_source_url.clone().unwrap();
        let managers = namui::managers();
        let image = managers.image_manager.try_load(&image_url).unwrap();
        let image_wh = image.size();

        let graph = match width_height {
            WidthHeight::Width => &mut layer.image.width,
            WidthHeight::Height => &mut layer.image.height,
        };
        let value = graph.get_value(playback_time).unwrap();
        let image_value = match width_height {
            WidthHeight::Width => image_wh.width,
            WidthHeight::Height => image_wh.height,
        };
        let current: f32 = (value * image_value).into();
        let next = current
            + (match plus_minus {
                PlusMinus::Plus => 1.0,
                PlusMinus::Minus => -1.0,
            }) * (match width_height {
                WidthHeight::Width => delta_in_real.x,
                WidthHeight::Height => delta_in_real.y,
            });
        let next_value = Percent::from(next / image_value);
        graph.put(
            KeyframePoint::new(playback_time, next_value),
            animation::KeyframeLine::Linear,
        );
    }
    fn update_xy(
        &self,
        layer: &mut Layer,
        playback_time: Time,
        delta_in_real: Xy<f32>,
        x_y: XY,
        plus_minus: PlusMinus,
    ) {
        let graph = match x_y {
            XY::X => &mut layer.image.x,
            XY::Y => &mut layer.image.y,
        };
        let value = graph.get_value(playback_time).unwrap();
        let current: f32 = value.into();
        let next = current
            + (match plus_minus {
                PlusMinus::Plus => 1.0,
                PlusMinus::Minus => -1.0,
            }) * (match x_y {
                XY::X => delta_in_real.x,
                XY::Y => delta_in_real.y,
            });
        let next_value = next.into();
        graph.put(
            KeyframePoint::new(playback_time, next_value),
            animation::KeyframeLine::Linear,
        );
    }
}
enum PlusMinus {
    Plus,
    Minus,
}
enum WidthHeight {
    Width,
    Height,
}

enum XY {
    X,
    Y,
}
