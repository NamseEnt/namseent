use super::*;
use crate::types::Act;
pub(crate) mod dragging;

impl WysiwygWindow {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::BackgroundClicked { mouse_xy } => {
                    if self.dragging.is_none() {
                        self.dragging = Some(Dragging::Background {
                            anchor_xy: *mouse_xy,
                        });
                    }
                }
                Event::MouseMoveIn { mouse_local_xy } => {
                    self.handle_dragging(*mouse_local_xy);
                }
                &Event::ShiftWheel { delta } => {
                    self.real_left_top_xy.x += px(delta);
                }
                &Event::Wheel { delta } => {
                    self.real_left_top_xy.y += px(delta);
                }
                Event::AltWheel {
                    delta,
                    mouse_local_xy,
                } => {
                    let next_real_px_per_screen_px = zoom(self.real_px_per_screen_px, *delta);

                    let real_xy_on_mouse_xy =
                        self.real_left_top_xy + self.real_px_per_screen_px * *mouse_local_xy;

                    let next_left_top_xy =
                        real_xy_on_mouse_xy - next_real_px_per_screen_px * *mouse_local_xy;

                    self.real_left_top_xy = next_left_top_xy;

                    self.real_px_per_screen_px = next_real_px_per_screen_px;
                }
                &Event::UpdateWh { wh } => {
                    self.last_wh = Some(wh);
                    self.center_viewport(wh);
                }
                &Event::SelectedLayerMouseDown {
                    ref layer_id,
                    anchor_xy,
                    ref keyframe_point_id,
                } => {
                    if self.dragging.is_none() {
                        if let Some(ticket) =
                            self.animation_history
                                .try_set_action(dragging::DragImageBodyAction {
                                    anchor_xy,
                                    last_mouse_local_xy: anchor_xy,
                                    layer_id: layer_id.clone(),
                                    keyframe_point_id: keyframe_point_id.clone(),
                                    real_px_per_screen_px: self.real_px_per_screen_px,
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
                    ref keyframe_point_id,
                    rotation_angle,
                } => {
                    if self.dragging.is_none() {
                        if let Some(ticket) = self.animation_history.try_set_action(
                            dragging::DragResizeCircleAction {
                                anchor_xy,
                                last_mouse_local_xy: anchor_xy,
                                layer_id: layer_id.clone(),
                                keyframe_point_id: keyframe_point_id.clone(),
                                real_px_per_screen_px: self.real_px_per_screen_px,
                                location,
                                rotation_angle,
                            },
                        ) {
                            self.dragging = Some(Dragging::ResizeCircle { ticket });
                        }
                    }
                }
                &Event::RotationToolMouseDown {
                    image_center_real_xy,
                    mouse_local_xy: mouse_real_xy,
                    ref keyframe_point_id,
                    ref layer_id,
                } => {
                    if self.dragging.is_none() {
                        if let Some(ticket) =
                            self.animation_history
                                .try_set_action(dragging::DragRotationAction {
                                    image_center_real_xy,
                                    start_mouse_real_xy: mouse_real_xy,
                                    end_mouse_real_xy: mouse_real_xy,
                                    keyframe_point_id: keyframe_point_id.clone(),
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

    fn center_viewport(&mut self, wh: Wh<Px>) {
        let viewport_center_in_real_px = Xy {
            x: px(1920.0) / 2.0,
            y: px(1080.0) / 2.0,
        };

        let window_center_in_real_px = (self.real_px_per_screen_px / 2.0) * wh.as_xy();

        self.real_left_top_xy = viewport_center_in_real_px - window_center_in_real_px;
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
