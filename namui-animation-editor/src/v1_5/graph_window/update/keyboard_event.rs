use super::*;

impl GraphWindow {
    pub(super) fn handle_key_down(&mut self, event: &KeyEvent) {
        self.handle_arrow_key_down(event);
        self.handle_delete_key_down(event);
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

        let animation = self.animation.read();
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
            }
            PropertyName::Y => todo!(),
            PropertyName::Width => todo!(),
            PropertyName::Height => todo!(),
        }

        namui::event::send(super::super::super::Event::UpdateLayer(Arc::new(layer)));
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
                        + PixelSize(mouse_over_row.mouse_local_xy.x) * self.context.time_per_pixel;

                    let next_time_per_pixel = zoom_time_per_pixel(
                        self.context.time_per_pixel,
                        match arrow {
                            Arrow::Left => 10.0,
                            Arrow::Right => -10.0,
                            _ => unreachable!(),
                        },
                    );

                    let next_start_at = time_at_mouse_position
                        - PixelSize(mouse_over_row.mouse_local_xy.x) * next_time_per_pixel;

                    self.context.time_per_pixel = next_time_per_pixel;
                    self.context.start_at = next_start_at;
                }
                Arrow::Top | Arrow::Bottom => match mouse_over_row.property_name {
                    PropertyName::X => {
                        let value_at_mouse_position = self.x_context.value_at_bottom
                            + self.x_context.value_per_pixel
                                * PixelSize(row_height - mouse_over_row.mouse_local_xy.y);

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
                                * PixelSize(row_height - mouse_over_row.mouse_local_xy.y);

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

    fn handle_delete_key_down(&self, event: &KeyEvent) {
        if event.code != Code::Delete {
            return;
        }

        if self.selected_point_address.is_none() {
            return;
        }
        let selected_point_address = self.selected_point_address.as_ref().unwrap();

        let animation = self.animation.read();
        let layer = animation
            .layers
            .iter()
            .find(|layer| layer.id.eq(&selected_point_address.layer_id));
        if layer.is_none() {
            return;
        }
        let mut layer = layer.unwrap().clone();
        layer.image.x.delete(&selected_point_address.point_id);
        namui::event::send(super::super::super::Event::UpdateLayer(Arc::new(layer)));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arrow {
    Left,
    Right,
    Top,
    Bottom,
}
