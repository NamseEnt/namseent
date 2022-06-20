use namui::{prelude::*, types::Time};
use namui_prebuilt::{table::*, *};
mod render;

pub struct WysiwygWindow {
    animation: crate::ReadOnlyLock<animation::Animation>,
    left_top_xy: Xy<f32>,
    mouse_drag_anchor_xy: Option<Xy<f32>>,
}

pub struct Props {
    pub wh: Wh<f32>,
}

enum Event {
    BackgroundClicked { mouse_xy: Xy<f32> },
    MouseMoveIn { mouse_xy: Xy<f32> },
}

impl WysiwygWindow {
    pub fn new(animation: crate::ReadOnlyLock<animation::Animation>) -> Self {
        Self {
            animation,
            left_top_xy: Xy { x: -5.0, y: -5.0 },
            mouse_drag_anchor_xy: None,
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
                        let delta = mouse_drag_anchor_xy - *mouse_xy;
                        self.left_top_xy = self.left_top_xy + delta;

                        self.mouse_drag_anchor_xy = Some(*mouse_xy);
                    }
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseUp(_) => self.mouse_drag_anchor_xy = None,
                _ => {}
            }
        }
    }
}
