use super::*;
impl GraphWindow {
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::GraphMouseMoveIn {
                    property_name,
                    local_xy,
                } => {
                    self.mouse_over_row = Some(MouseOverRow {
                        property_name: *property_name,
                        local_xy: *local_xy,
                    });
                    match property_name {
                        PropertyName::X => {
                            self.x_context.mouse_local_xy = Some(*local_xy);
                        }
                        PropertyName::Y => todo!(),
                        PropertyName::Width => todo!(),
                        PropertyName::Height => todo!(),
                    }
                }
                Event::GraphMouseMoveOut { property_name } => {
                    if self
                        .mouse_over_row
                        .as_ref()
                        .map(|row| row.property_name == *property_name)
                        == Some(true)
                    {
                        self.mouse_over_row = None;
                    }
                    match property_name {
                        PropertyName::X => {
                            self.x_context.mouse_local_xy = None;
                        }
                        PropertyName::Y => todo!(),
                        PropertyName::Width => todo!(),
                        PropertyName::Height => todo!(),
                    }
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
                } => match property_name {
                    PropertyName::X => {
                        let value_at_mouse_position = self.x_context.value_at_bottom
                            + self.x_context.value_per_pixel
                                * PixelSize(row_wh.height - anchor_xy.y);

                        let next_value_per_pixel =
                            zoom_pixel_size_per_pixel(self.x_context.value_per_pixel, delta.into());

                        let next_value_at_bottom = value_at_mouse_position
                            - next_value_per_pixel * PixelSize(row_wh.height - anchor_xy.y);

                        self.x_context.value_per_pixel = next_value_per_pixel;
                        self.x_context.value_at_bottom = next_value_at_bottom;
                    }
                    PropertyName::Y => todo!(),
                    PropertyName::Width => todo!(),
                    PropertyName::Height => todo!(),
                },
                Event::RowHeightChange { row_height } => {
                    self.row_height = Some(*row_height);
                }
                Event::TimelineTimeRulerClicked {
                    click_position_in_time,
                } => {
                    // TODO
                }
                Event::GraphMouseRightClick {
                    property_name,
                    mouse_local_xy,
                    row_wh,
                    layer_id,
                } => {
                    let time = self.context.start_at
                        + PixelSize(mouse_local_xy.x) * self.context.time_per_pixel;

                    let animation = self.animation.read().unwrap();
                    let layer = animation.layers.iter().find(|layer| layer.id.eq(layer_id));
                    if layer.is_none() {
                        return;
                    }
                    let mut layer = layer.unwrap().clone();
                    match property_name {
                        PropertyName::X => layer.image.x.put(
                            animation::KeyframePoint::new(
                                time,
                                self.x_context.value_at_bottom
                                    + self.x_context.value_per_pixel
                                        * PixelSize(row_wh.height - mouse_local_xy.y),
                            ),
                            animation::KeyframeLine::Linear,
                        ),
                        PropertyName::Y => todo!(),
                        PropertyName::Width => todo!(),
                        PropertyName::Height => todo!(),
                    }
                    namui::event::send(super::super::Event::UpdateLayer(Arc::new(layer)));
                }
                Event::GraphPointClick {
                    layer_id,
                    property_name,
                    point_id,
                } => {
                    self.selected_point_address = Some(SelectedPointAddress {
                        layer_id: layer_id.clone(),
                        property_name: *property_name,
                        point_id: point_id.clone(),
                    });
                }
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            if let NamuiEvent::KeyDown(event) = event {
                self.handle_arrow_key_down(event);
            }
        }
    }

    fn handle_arrow_key_down(&mut self, event: &KeyEvent) {
        let arrow = match event.code {
            Code::ArrowLeft => Arrow::Left,
            Code::ArrowRight => Arrow::Right,
            Code::ArrowUp => Arrow::Top,
            Code::ArrowDown => Arrow::Bottom,
            _ => return,
        };

        self.handle_point_move(arrow);
        self.handle_graph_move_and_zoom(arrow);
    }

    fn handle_point_move(&self, arrow: Arrow) {
        if self.selected_point_address.is_none() {
            return;
        }
        let selected_point_address = self.selected_point_address.as_ref().unwrap();

        let animation = self.animation.read().unwrap();
        let layer = animation
            .layers
            .iter()
            .find(|layer| layer.id.eq(&selected_point_address.layer_id));
        if layer.is_none() {
            return;
        }
        let mut layer = layer.unwrap().clone();

        match selected_point_address.property_name {
            PropertyName::X => {
                let mut selected_point = layer
                    .image
                    .x
                    .get_point(&selected_point_address.point_id)
                    .unwrap()
                    .clone();

                match arrow {
                    Arrow::Left | Arrow::Right => {
                        selected_point.time += self.context.time_per_pixel
                            * PixelSize(match arrow {
                                Arrow::Left => -1.0,
                                Arrow::Right => 1.0,
                                _ => unreachable!(),
                            });
                    }
                    Arrow::Top | Arrow::Bottom => {
                        selected_point.value += self.x_context.value_per_pixel
                            * PixelSize({
                                match arrow {
                                    Arrow::Top => 1.0,
                                    Arrow::Bottom => -1.0,
                                    _ => unreachable!(),
                                }
                            });
                    }
                };
                layer
                    .image
                    .x
                    .put(selected_point, animation::KeyframeLine::Linear);
                namui::log!("layer.image.x: {:?}", layer.image.x);
            }
            PropertyName::Y => todo!(),
            PropertyName::Width => todo!(),
            PropertyName::Height => todo!(),
        }

        namui::event::send(super::super::Event::UpdateLayer(Arc::new(layer)));
    }

    fn handle_graph_move_and_zoom(&mut self, arrow: Arrow) {
        if self.row_height.is_none() {
            return;
        }
        let row_height = self.row_height.unwrap();
        if self.mouse_over_row.is_none() {
            return;
        }
        let mouse_over_row = self.mouse_over_row.as_ref().unwrap();

        let managers = namui::managers();
        if managers
            .keyboard_manager
            .any_code_press([Code::AltLeft, Code::AltRight])
        {
            match arrow {
                Arrow::Left | Arrow::Right => {
                    let time_at_mouse_position = self.context.start_at
                        + PixelSize(mouse_over_row.local_xy.x) * self.context.time_per_pixel;

                    let next_time_per_pixel = zoom_time_per_pixel(
                        self.context.time_per_pixel,
                        match arrow {
                            Arrow::Left => 10.0,
                            Arrow::Right => -10.0,
                            _ => unreachable!(),
                        },
                    );

                    let next_start_at = time_at_mouse_position
                        - PixelSize(mouse_over_row.local_xy.x) * next_time_per_pixel;

                    self.context.time_per_pixel = next_time_per_pixel;
                    self.context.start_at = next_start_at;
                }
                Arrow::Top | Arrow::Bottom => match mouse_over_row.property_name {
                    PropertyName::X => {
                        let value_at_mouse_position = self.x_context.value_at_bottom
                            + self.x_context.value_per_pixel
                                * PixelSize(row_height - mouse_over_row.local_xy.y);

                        let next_value_per_pixel = zoom_pixel_size_per_pixel(
                            self.x_context.value_per_pixel,
                            match arrow {
                                Arrow::Top => 10.0,
                                Arrow::Bottom => -10.0,
                                _ => unreachable!(),
                            },
                        );

                        let next_value_at_bottom = value_at_mouse_position
                            - next_value_per_pixel
                                * PixelSize(row_height - mouse_over_row.local_xy.y);

                        self.x_context.value_per_pixel = next_value_per_pixel;
                        self.x_context.value_at_bottom = next_value_at_bottom;
                    }
                    PropertyName::Y => todo!(),
                    PropertyName::Width => todo!(),
                    PropertyName::Height => todo!(),
                },
            }
        } else if managers
            .keyboard_manager
            .any_code_press([Code::ShiftLeft, Code::ShiftRight])
        {
            match arrow {
                Arrow::Left | Arrow::Right => {
                    self.context.start_at += self.context.time_per_pixel
                        * PixelSize(match arrow {
                            Arrow::Left => -10.0,
                            Arrow::Right => 10.0,
                            _ => unreachable!(),
                        });
                }
                Arrow::Top | Arrow::Bottom => match mouse_over_row.property_name {
                    PropertyName::X => {
                        self.x_context.value_at_bottom += self.x_context.value_per_pixel
                            * PixelSize(match arrow {
                                Arrow::Top => 10.0,
                                Arrow::Bottom => -10.0,
                                _ => unreachable!(),
                            });
                    }
                    PropertyName::Y => todo!(),
                    PropertyName::Width => todo!(),
                    PropertyName::Height => todo!(),
                },
            }
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arrow {
    Left,
    Right,
    Top,
    Bottom,
}
