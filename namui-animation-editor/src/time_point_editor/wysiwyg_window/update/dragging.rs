use super::*;
use namui::{
    animation::{KeyframePoint, Layer},
    types::Percent,
};

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
            &Dragging::ResizeCircle { ticket } => {
                self.animation_history
                    .update_action(ticket, |action: &mut DragResizeCircle| {
                        action.real_pixel_size_per_screen_pixel_size =
                            self.real_pixel_size_per_screen_pixel_size;
                        action.last_mouse_local_xy = mouse_local_xy;
                    })
                    .unwrap();
            }
            &Dragging::ImageBody { ticket } => {
                self.animation_history
                    .update_action(ticket, |action: &mut DragImageBody| {
                        action.real_pixel_size_per_screen_pixel_size =
                            self.real_pixel_size_per_screen_pixel_size;
                        action.last_mouse_local_xy = mouse_local_xy;
                    })
                    .unwrap();
            }
        }
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

fn update_xy(
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
fn update_size(
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
pub(super) struct DragImageBody {
    pub layer_id: String,
    pub anchor_xy: Xy<f32>,
    pub last_mouse_local_xy: Xy<f32>,
    pub playback_time: Time,
    pub real_pixel_size_per_screen_pixel_size: f32,
}
impl Act<Animation> for DragImageBody {
    fn act(&self, state: &Animation) -> Result<Animation, Box<dyn std::error::Error>> {
        let mut animation = state.clone();
        if let Some(layer) = animation
            .layers
            .iter_mut()
            .find(|layer| layer.id.eq(&self.layer_id))
        {
            let delta_in_real = self.real_pixel_size_per_screen_pixel_size
                * (self.last_mouse_local_xy - self.anchor_xy);

            update_xy(
                layer,
                self.playback_time,
                delta_in_real,
                XY::X,
                PlusMinus::Plus,
            );
            update_xy(
                layer,
                self.playback_time,
                delta_in_real,
                XY::Y,
                PlusMinus::Plus,
            );

            Ok(animation)
        } else {
            Err("layer not found".into())
        }
    }
}
pub(super) struct DragResizeCircle {
    pub layer_id: String,
    pub anchor_xy: Xy<f32>,
    pub last_mouse_local_xy: Xy<f32>,
    pub playback_time: Time,
    pub real_pixel_size_per_screen_pixel_size: f32,
    pub location: ResizeCircleLocation,
}
impl Act<Animation> for DragResizeCircle {
    fn act(&self, state: &Animation) -> Result<Animation, Box<dyn std::error::Error>> {
        let mut animation = state.clone();
        if let Some(layer) = animation
            .layers
            .iter_mut()
            .find(|layer| layer.id.eq(&self.layer_id))
        {
            let delta_in_real = self.real_pixel_size_per_screen_pixel_size
                * (self.last_mouse_local_xy - self.anchor_xy);

            match self.location {
                ResizeCircleLocation::LeftTop
                | ResizeCircleLocation::Top
                | ResizeCircleLocation::RightTop => {
                    update_size(
                        layer,
                        self.playback_time,
                        delta_in_real,
                        WidthHeight::Height,
                        PlusMinus::Minus,
                    );
                    update_xy(
                        layer,
                        self.playback_time,
                        delta_in_real,
                        XY::Y,
                        PlusMinus::Plus,
                    );
                }
                ResizeCircleLocation::LeftBottom
                | ResizeCircleLocation::Bottom
                | ResizeCircleLocation::RightBottom => {
                    update_size(
                        layer,
                        self.playback_time,
                        delta_in_real,
                        WidthHeight::Height,
                        PlusMinus::Plus,
                    );
                }
                _ => {}
            }

            match self.location {
                ResizeCircleLocation::LeftTop
                | ResizeCircleLocation::Left
                | ResizeCircleLocation::LeftBottom => {
                    update_size(
                        layer,
                        self.playback_time,
                        delta_in_real,
                        WidthHeight::Width,
                        PlusMinus::Minus,
                    );
                    update_xy(
                        layer,
                        self.playback_time,
                        delta_in_real,
                        XY::X,
                        PlusMinus::Plus,
                    );
                }
                ResizeCircleLocation::RightTop
                | ResizeCircleLocation::Right
                | ResizeCircleLocation::RightBottom => {
                    update_size(
                        layer,
                        self.playback_time,
                        delta_in_real,
                        WidthHeight::Width,
                        PlusMinus::Plus,
                    );
                }
                _ => {}
            }

            Ok(animation)
        } else {
            Err("layer not found".into())
        }
    }
}
