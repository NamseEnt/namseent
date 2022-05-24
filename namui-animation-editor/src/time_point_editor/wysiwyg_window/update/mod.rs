use super::*;
use crate::types::Act;
use namui::animation::Animation;
mod dragging;

impl WysiwygWindow {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::BackgroundClicked { mouse_xy } => {
                    if self.dragging.is_none() {
                        self.dragging = Some(Dragging::Background {
                            anchor_xy: mouse_xy.clone(),
                        });
                    }
                }
                Event::MouseMoveIn { mouse_local_xy } => {
                    self.handle_dragging(*mouse_local_xy);
                }
                Event::ShiftWheel { delta } => {
                    self.real_left_top_xy.x += delta;
                }
                Event::Wheel { delta } => {
                    self.real_left_top_xy.y += delta;
                }
                Event::AltWheel {
                    delta,
                    mouse_local_xy,
                } => {
                    let next_real_pixel_size_per_screen_pixel_size =
                        zoom(self.real_pixel_size_per_screen_pixel_size, *delta);

                    let real_xy_on_mouse_xy = self.real_left_top_xy
                        + self.real_pixel_size_per_screen_pixel_size * *mouse_local_xy;

                    let next_left_top_xy = real_xy_on_mouse_xy
                        - next_real_pixel_size_per_screen_pixel_size * *mouse_local_xy;

                    self.real_left_top_xy = next_left_top_xy;

                    self.real_pixel_size_per_screen_pixel_size =
                        next_real_pixel_size_per_screen_pixel_size;
                }
                Event::UpdateWh { wh } => {
                    self.last_wh = Some(*wh);
                    self.center_viewport(*wh);
                }
                &Event::SelectedLayerMouseDown {
                    ref layer_id,
                    anchor_xy,
                    playback_time,
                } => {
                    if self.dragging.is_none() {
                        if let Some(ticket) =
                            self.animation_history
                                .try_set_action(dragging::DragImageBodyAction {
                                    anchor_xy,
                                    last_mouse_local_xy: anchor_xy,
                                    layer_id: layer_id.clone(),
                                    playback_time,
                                    real_pixel_size_per_screen_pixel_size: self
                                        .real_pixel_size_per_screen_pixel_size,
                                })
                        {
                            self.dragging = Some(Dragging::ImageBody { ticket });
                        }
                    }
                }
                &Event::ResizeCircleMouseDown {
                    ref layer_id,
                    location,
                    anchor_xy,
                    playback_time,
                    rotation_radian,
                } => {
                    if self.dragging.is_none() {
                        if let Some(ticket) = self.animation_history.try_set_action(
                            dragging::DragResizeCircleAction {
                                anchor_xy,
                                last_mouse_local_xy: anchor_xy,
                                layer_id: layer_id.clone(),
                                playback_time,
                                real_pixel_size_per_screen_pixel_size: self
                                    .real_pixel_size_per_screen_pixel_size,
                                location,
                                rotation_radian,
                            },
                        ) {
                            self.dragging = Some(Dragging::ResizeCircle { ticket });
                        }
                    }
                }
                &Event::RotationToolMouseDown {
                    image_center_real_xy,
                    mouse_local_xy: mouse_real_xy,
                    playback_time,
                    ref layer_id,
                } => {
                    if self.dragging.is_none() {
                        if let Some(ticket) =
                            self.animation_history
                                .try_set_action(dragging::DragRotationAction {
                                    image_center_real_xy,
                                    start_mouse_real_xy: mouse_real_xy,
                                    end_mouse_real_xy: mouse_real_xy,
                                    playback_time,
                                    layer_id: layer_id.clone(),
                                })
                        {
                            self.dragging = Some(Dragging::Rotation { ticket });
                        }
                    }
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseUp(_) => {
                    match &self.dragging {
                        Some(dragging) => match dragging {
                            Dragging::ImageBody { ticket }
                            | Dragging::ResizeCircle { ticket }
                            | Dragging::Rotation { ticket } => {
                                self.animation_history.act(*ticket).unwrap();
                            }
                            _ => {}
                        },
                        None => {}
                    };
                    self.dragging = None;
                }
                NamuiEvent::KeyDown(event) => {
                    if let Some(wh) = self.last_wh {
                        if event.code == Code::Home {
                            self.center_viewport(wh);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn center_viewport(&mut self, wh: Wh<f32>) {
        let viewport_center_in_real_pixel = Xy {
            x: 1920.0 / 2.0,
            y: 1080.0 / 2.0,
        };

        let window_center_in_real_pixel = self.real_pixel_size_per_screen_pixel_size / 2.0
            * Xy {
                x: wh.width,
                y: wh.height,
            };

        self.real_left_top_xy = viewport_center_in_real_pixel - window_center_in_real_pixel;
    }
}

fn zoom(target: f32, delta: f32) -> f32 {
    const STEP: f32 = 400.0;
    const MIN: f32 = 0.01;
    const MAX: f32 = 4.0;

    let wheel = STEP * (target / 10.0).log2();

    let next_wheel = wheel + delta;

    let zoomed = namui::math::num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);

    zoomed
}
