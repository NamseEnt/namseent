use namui::{prelude::*, types::Time};
use namui_prebuilt::{table::*, *};
mod render;

pub struct WysiwygWindow {
    animation: crate::ReadOnlyLock<animation::Animation>,
    real_left_top_xy: Xy<f32>,
    mouse_drag_anchor_xy: Option<Xy<f32>>,
    real_pixel_size_per_screen_pixel_size: f32,
    last_wh: Option<Wh<f32>>,
    playback_time: Time,
}

pub struct Props {
    pub wh: Wh<f32>,
}

enum Event {
    BackgroundClicked { mouse_xy: Xy<f32> },
    MouseMoveIn { mouse_xy: Xy<f32> },
    ShiftWheel { delta: f32 },
    Wheel { delta: f32 },
    AltWheel { delta: f32, mouse_local_xy: Xy<f32> },
    UpdateWh { wh: Wh<f32> },
}

impl WysiwygWindow {
    pub fn new(animation: crate::ReadOnlyLock<animation::Animation>) -> Self {
        Self {
            animation,
            real_left_top_xy: Xy { x: -5.0, y: -5.0 },
            mouse_drag_anchor_xy: None,
            real_pixel_size_per_screen_pixel_size: 2.0,
            last_wh: None,
            playback_time: Time::zero(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::BackgroundClicked { mouse_xy } => {
                    self.mouse_drag_anchor_xy = Some(*mouse_xy)
                }
                Event::MouseMoveIn { mouse_xy } => {
                    if let Some(mouse_drag_anchor_xy) = self.mouse_drag_anchor_xy {
                        let delta = self.real_pixel_size_per_screen_pixel_size
                            * (mouse_drag_anchor_xy - *mouse_xy);
                        self.real_left_top_xy = self.real_left_top_xy + delta;

                        self.mouse_drag_anchor_xy = Some(*mouse_xy);
                    }
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
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseUp(_) => self.mouse_drag_anchor_xy = None,
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
