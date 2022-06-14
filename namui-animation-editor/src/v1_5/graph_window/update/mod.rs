use super::*;
mod keyboard_event;

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
                        + PixelSize(anchor_xy.x) * self.context.time_per_pixel;

                    let next_time_per_pixel =
                        zoom_time_per_pixel(self.context.time_per_pixel, delta.into());

                    let next_start_at =
                        time_at_mouse_position - PixelSize(anchor_xy.x) * next_time_per_pixel;

                    self.context.time_per_pixel = next_time_per_pixel;
                    self.context.start_at = next_start_at;
                }
                Event::GraphCtrlMouseWheel {
                    delta,
                    property_name,
                    mouse_local_xy: anchor_xy,
                    row_wh,
                } => {
                    let property_context = self.get_property_context_mut(*property_name);
                    let value_at_mouse_position = property_context.value_at_bottom
                        + property_context.value_per_pixel * PixelSize(row_wh.height - anchor_xy.y);

                    let next_value_per_pixel =
                        zoom_pixel_size_per_pixel(property_context.value_per_pixel, delta.into());

                    let next_value_at_bottom = value_at_mouse_position
                        - next_value_per_pixel * PixelSize(row_wh.height - anchor_xy.y);

                    property_context.value_per_pixel = next_value_per_pixel;
                    property_context.value_at_bottom = next_value_at_bottom;
                }
                Event::RowHeightChange { row_height } => {
                    self.row_height = Some(*row_height);
                }
                Event::TimelineTimeRulerClicked {
                    click_position_in_time,
                } => {
                    // TODO
                }
                Event::GraphMouseRightDown {
                    property_name,
                    mouse_local_xy,
                    row_wh,
                    layer_id,
                } => {
                    let time = self.context.start_at
                        + PixelSize(mouse_local_xy.x) * self.context.time_per_pixel;

                    let mut layer = {
                        let animation = self.animation.read();
                        let layer = animation.layers.iter().find(|layer| layer.id.eq(layer_id));
                        if layer.is_none() {
                            return;
                        }
                        layer.unwrap().clone()
                    };

                    let property_context = self.get_property_context_mut(*property_name);
                    let graph = get_keyframe_graph_mut(&mut layer, *property_name);
                    graph.put(
                        animation::KeyframePoint::new(
                            time,
                            property_context.value_at_bottom
                                + property_context.value_per_pixel
                                    * PixelSize(row_wh.height - mouse_local_xy.y),
                        ),
                        animation::KeyframeLine::Linear,
                    );
                    namui::event::send(super::super::Event::UpdateLayer(Arc::new(layer)));
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
                        })
                    }
                    self.playback_time = self.context.start_at
                        + PixelSize(mouse_local_xy.x) * self.context.time_per_pixel;
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
                property_name,
                last_mouse_local_xy,
            } => {
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

        let time_on_x =
            self.context.start_at + PixelSize(mouse_local_xy.x) * self.context.time_per_pixel;

        let property_context = self.get_property_context(property_name);
        let graph = get_keyframe_graph_mut(&mut layer, property_name);

        let mut point = graph.get_point(&point_address.point_id).unwrap().clone();

        let value_on_y = property_context.value_at_bottom
            + property_context.value_per_pixel * PixelSize(row_wh.height - mouse_local_xy.y);

        point.time = time_on_x;
        point.value = value_on_y;

        graph.put(point, animation::KeyframeLine::Linear);

        namui::event::send(super::super::Event::UpdateLayer(Arc::new(layer)));
    }
    fn handle_background_dragging(
        &mut self,
        property_name: PropertyName,
        last_mouse_local_xy: Xy<f32>,
        mouse_local_xy: Xy<f32>,
    ) {
        let mouse_delta_xy = mouse_local_xy - last_mouse_local_xy;

        self.context.start_at -= self.context.time_per_pixel * PixelSize(mouse_delta_xy.x);

        let property_context = self.get_property_context_mut(property_name);

        property_context.value_at_bottom +=
            property_context.value_per_pixel * PixelSize(mouse_delta_xy.y);

        self.dragging = Some(Dragging::Background {
            property_name,
            last_mouse_local_xy: mouse_local_xy,
        });
    }
    fn get_property_context_mut(
        &mut self,
        property_name: PropertyName,
    ) -> &mut PropertyContext<PixelSize> {
        match property_name {
            PropertyName::X => &mut self.x_context,
            PropertyName::Y => &mut self.y_context,
            PropertyName::Width => &mut self.width_context,
            PropertyName::Height => &mut self.height_context,
        }
    }
    fn get_property_context(&self, property_name: PropertyName) -> &PropertyContext<PixelSize> {
        match property_name {
            PropertyName::X => &self.x_context,
            PropertyName::Y => &self.y_context,
            PropertyName::Width => &self.width_context,
            PropertyName::Height => &self.height_context,
        }
    }
}
fn get_keyframe_graph_mut(
    layer: &mut Layer,
    property_name: PropertyName,
) -> &mut KeyframeGraph<PixelSize> {
    match property_name {
        PropertyName::X => &mut layer.image.x,
        PropertyName::Y => &mut layer.image.y,
        PropertyName::Width => &mut layer.image.width,
        PropertyName::Height => &mut layer.image.height,
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

fn zoom_pixel_size_per_pixel(
    target: ValuePerPixel<PixelSize>,
    delta: f32,
) -> ValuePerPixel<PixelSize> {
    const STEP: f32 = 400.0;
    const MIN: f32 = 1.0;
    const MAX: f32 = 100.0;

    let wheel = STEP * (target.value / target.pixel_size / 10.0).log2();

    let next_wheel = wheel + delta;

    let zoomed = namui::math::num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);

    ValuePerPixel {
        value: zoomed.into(),
        pixel_size: 1.0_f32.into(),
    }
}
