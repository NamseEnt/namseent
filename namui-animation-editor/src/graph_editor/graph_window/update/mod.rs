mod context;
mod keyboard_event;
mod move_by;
mod move_to;
use self::move_to::MovePointToAction;
use super::*;
use crate::types::Act;
use namui::animation::Animation;

impl GraphWindow {
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                &Event::GraphMouseMoveIn {
                    property_name,
                    mouse_xy_in_row,
                } => {
                    self.mouse_over_row = Some(MouseOverRow {
                        property_name,
                        mouse_xy_in_row,
                    });

                    self.handle_dragging_move(property_name, mouse_xy_in_row);
                }
                Event::GraphMouseMoveOut => {
                    self.mouse_over_row = None;
                }
                Event::GraphShiftMouseWheel { delta } => {
                    self.context.start_at += delta * self.context.time_per_px;
                }
                Event::GraphMouseWheel {
                    delta,
                    property_name,
                } => {
                    self.move_property_context_by(*property_name, delta.to_f32().unwrap());
                }
                Event::GraphAltMouseWheel {
                    delta,
                    mouse_local_xy: anchor_xy,
                } => {
                    let time_at_mouse_position =
                        self.context.start_at + Px::from(anchor_xy.x) * self.context.time_per_px;

                    let next_time_per_px =
                        zoom_time_per_px(self.context.time_per_px, delta.to_f32().unwrap());

                    let next_start_at =
                        time_at_mouse_position - Px::from(anchor_xy.x) * next_time_per_px;

                    self.context.time_per_px = next_time_per_px;
                    self.context.start_at = next_start_at;
                }
                Event::GraphCtrlMouseWheel {
                    delta,
                    property_name,
                    mouse_local_xy,
                    row_wh,
                } => {
                    self.zoom_property_context(
                        *property_name,
                        delta.to_f32().unwrap(),
                        mouse_local_xy.y,
                        row_wh.height,
                    );
                }
                &Event::GraphPointMouseDown {
                    ref point_address,
                    row_height,
                    y_in_row,
                } => {
                    if self.dragging.is_none() {
                        self.selected_point_address = Some(point_address.clone());

                        let action = self.get_move_to_action(point_address, row_height, y_in_row);
                        if let Some(ticket) = self.animation_history.try_set_action(action) {
                            self.dragging = Some(Dragging::Point { ticket });
                        }
                    }
                }
                Event::GraphMouseLeftDown {
                    property_name,
                    mouse_local_xy,
                } => {
                    if self.dragging.is_none() {
                        self.dragging = Some(Dragging::Background {
                            last_mouse_local_xy: *mouse_local_xy,
                            property_name: *property_name,
                        });
                        self.selected_point_address = None;
                    }
                    namui::event::send(crate::graph_editor::Event::SetPlaybackTime(
                        self.context.start_at
                            + Px::from(mouse_local_xy.x) * self.context.time_per_px,
                    ));
                }
                &Event::KeyboardKeyDown { code, row_height } => {
                    self.handle_key_down(code, row_height)
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseUp(_) => {
                    match &self.dragging {
                        Some(dragging) => match dragging {
                            &Dragging::Point { ticket, .. } => {
                                self.animation_history.act(ticket).unwrap();
                            }
                            _ => {}
                        },
                        None => {}
                    }
                    self.dragging = None;
                }
                _ => {}
            }
        }
    }

    fn handle_dragging_move(&mut self, property_name: PropertyName, mouse_xy_in_row: Xy<f32>) {
        if self.dragging.is_none() {
            return;
        }

        let dragging = self.dragging.clone().unwrap();

        match dragging {
            Dragging::Point { ticket, .. } => self
                .animation_history
                .update_action(ticket, |action: &mut MovePointToAction| {
                    action.y_in_row = Px::from_f32(mouse_xy_in_row.y).unwrap();
                })
                .unwrap(),
            Dragging::Background {
                property_name: dragging_property_name,
                last_mouse_local_xy,
            } => {
                if dragging_property_name != property_name {
                    return;
                }
                self.handle_background_dragging(property_name, last_mouse_local_xy, mouse_xy_in_row)
            }
        };
    }

    fn handle_background_dragging(
        &mut self,
        property_name: PropertyName,
        last_mouse_local_xy: Xy<f32>,
        mouse_local_xy: Xy<f32>,
    ) {
        let mouse_delta_xy = mouse_local_xy - last_mouse_local_xy;

        self.context.start_at -= self.context.time_per_px * Px::from(mouse_delta_xy.x);
        self.move_property_context_by(property_name, mouse_delta_xy.y);

        self.dragging = Some(Dragging::Background {
            property_name,
            last_mouse_local_xy: mouse_local_xy,
        });
    }
}

fn zoom_time_per_px(target: TimePerPx, delta: f32) -> TimePerPx {
    const STEP: f32 = 400.0;
    const MIN: f32 = 10.0;
    const MAX: f32 = 1000.0;

    let ms_per_px = (target * Px::from(1.0f32)).as_millis();

    let wheel = STEP * (ms_per_px / 10.0).log2();

    let next_wheel = wheel + delta;

    let zoomed = namui::math::num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);
    Time::Ms(zoomed) / Px::from(1.0f32)
}
