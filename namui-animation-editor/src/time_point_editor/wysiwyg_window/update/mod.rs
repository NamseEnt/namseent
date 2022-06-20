use super::*;
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
                    self.mouse_local_xy = Some(*mouse_local_xy);
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
                Event::LayerClicked { layer_id } => {
                    self.selected_layer_id = Some(layer_id.clone());
                }
                &Event::ResizeCircleClicked {
                    location,
                    anchor_xy,
                } => {
                    if self.dragging.is_none() {
                        self.dragging = Some(Dragging::ResizeCircle {
                            location,
                            anchor_xy,
                        });
                    }
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseUp(_) => self.dragging = None,
                NamuiEvent::KeyDown(event) => {
                    if let Some(wh) = self.last_wh {
                        if event.code == Code::Space {
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
