use super::*;
mod context;
mod keyboard_event;
mod layer;

impl GraphWindow {
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::GraphMouseMoveIn {
                    property_name,
                    mouse_local_xy,
                    row_wh,
                } => {
                    self.mouse_over_row = Some(MouseOverRow {
                        property_name: *property_name,
                        mouse_local_xy: *mouse_local_xy,
                    });

                    self.handle_dragging_move(*property_name, *mouse_local_xy, *row_wh);
                }
                Event::GraphMouseMoveOut => {
                    self.mouse_over_row = None;
                }
                Event::GraphShiftMouseWheel { delta } => {
                    self.context.start_at += delta * self.context.time_per_pixel;
                }
                Event::GraphAltMouseWheel {
                    delta,
                    mouse_local_xy: anchor_xy,
                } => {
                    let time_at_mouse_position = self.context.start_at
                        + PixelSize::from(anchor_xy.x) * self.context.time_per_pixel;

                    let next_time_per_pixel =
                        zoom_time_per_pixel(self.context.time_per_pixel, delta.into());

                    let next_start_at =
                        time_at_mouse_position - PixelSize::from(anchor_xy.x) * next_time_per_pixel;

                    self.context.time_per_pixel = next_time_per_pixel;
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
                        delta.into(),
                        mouse_local_xy.y,
                        row_wh.height,
                    );
                }
                Event::RowHeightChange { row_height } => {
                    self.row_height = Some(*row_height);
                }
                Event::GraphMouseRightDown {
                    property_name,
                    mouse_local_xy,
                    row_wh,
                    layer_id,
                } => {
                    let mut layer = {
                        let animation = self.animation.read();
                        let layer = animation.layers.iter().find(|layer| layer.id.eq(layer_id));
                        if layer.is_none() {
                            return;
                        }
                        layer.unwrap().clone()
                    };

                    self.add_point_into_xy(&mut layer, *property_name, *mouse_local_xy, *row_wh);

                    namui::event::send(crate::Event::UpdateLayer(Arc::new(layer)));
                }
                Event::GraphPointMouseDown { point_address } => {
                    if self.dragging.is_none() {
                        self.selected_point_address = Some(point_address.clone());
                        self.dragging = Some(Dragging::Point(point_address.clone()));
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
                            + PixelSize::from(mouse_local_xy.x) * self.context.time_per_pixel,
                    ));
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::KeyDown(event) => self.handle_key_down(event),
                NamuiEvent::MouseUp(_) => {
                    self.dragging = None;
                }
                _ => {}
            }
        }
    }

    fn handle_dragging_move(
        &mut self,
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    ) {
        if self.dragging.is_none() {
            return;
        }

        let dragging = self.dragging.clone().unwrap();

        match dragging {
            Dragging::Point(point_address) => {
                self.handle_point_dragging(&point_address, property_name, mouse_local_xy, row_wh)
            }
            Dragging::Background {
                property_name: dragging_property_name,
                last_mouse_local_xy,
            } => {
                if dragging_property_name != property_name {
                    return;
                }
                self.handle_background_dragging(property_name, last_mouse_local_xy, mouse_local_xy)
            }
        };
    }

    fn handle_point_dragging(
        &self,
        point_address: &PointAddress,
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    ) {
        if point_address.property_name != property_name {
            return;
        }

        let animation = self.animation.read();

        let layer = animation
            .layers
            .iter()
            .find(|layer| layer.id.eq(&point_address.layer_id));
        if layer.is_none() {
            return;
        }
        let mut layer = layer.unwrap().clone();

        self.move_point_into_xy(
            &mut layer,
            property_name,
            &point_address.point_id,
            mouse_local_xy,
            row_wh,
        );

        namui::event::send(crate::Event::UpdateLayer(Arc::new(layer)));
    }
    fn handle_background_dragging(
        &mut self,
        property_name: PropertyName,
        last_mouse_local_xy: Xy<f32>,
        mouse_local_xy: Xy<f32>,
    ) {
        let mouse_delta_xy = mouse_local_xy - last_mouse_local_xy;

        self.context.start_at -= self.context.time_per_pixel * PixelSize::from(mouse_delta_xy.x);
        self.move_property_context_by(property_name, mouse_delta_xy.y);

        self.dragging = Some(Dragging::Background {
            property_name,
            last_mouse_local_xy: mouse_local_xy,
        });
    }
}

fn zoom_time_per_pixel(target: TimePerPixel, delta: f32) -> TimePerPixel {
    const STEP: f32 = 400.0;
    const MIN: f32 = 10.0;
    const MAX: f32 = 1000.0;

    let ms_per_pixel = target.ms_per_pixel();

    let wheel = STEP * (ms_per_pixel / 10.0).log2();

    let next_wheel = wheel + delta;

    let zoomed = namui::math::num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);
    TimePerPixel::from_ms_per_pixel(zoomed)
}
