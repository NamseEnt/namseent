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

        let delta_xy = Xy {
            x: match arrow {
                Arrow::Left => -1.0,
                Arrow::Right => 1.0,
                _ => 0.0,
            },
            y: match arrow {
                Arrow::Top => 1.0,
                Arrow::Bottom => -1.0,
                _ => 0.0,
            },
        };

        self.move_point_by_xy(
            &mut layer,
            selected_point_address.property_name,
            &selected_point_address.point_id,
            delta_xy,
        );

        namui::event::send(crate::Event::UpdateLayer(Arc::new(layer)));
    }

    fn handle_graph_move_and_zoom(&mut self, arrow: Arrow) {
        if self.row_height.is_none() {
            return;
        }
        let row_height = self.row_height.unwrap();
        if self.mouse_over_row.is_none() {
            return;
        }
        let mouse_over_row = self.mouse_over_row.clone().unwrap();

        if namui::system::keyboard::any_code_press([Code::AltLeft, Code::AltRight]) {
            match arrow {
                Arrow::Left | Arrow::Right => {
                    let time_at_mouse_position = self.context.start_at
                        + PixelSize::from(mouse_over_row.mouse_local_xy.x)
                            * self.context.time_per_pixel;

                    let next_time_per_pixel = zoom_time_per_pixel(
                        self.context.time_per_pixel,
                        match arrow {
                            Arrow::Left => 10.0,
                            Arrow::Right => -10.0,
                            _ => unreachable!(),
                        },
                    );

                    let next_start_at = time_at_mouse_position
                        - PixelSize::from(mouse_over_row.mouse_local_xy.x) * next_time_per_pixel;

                    self.context.time_per_pixel = next_time_per_pixel;
                    self.context.start_at = next_start_at;
                }
                Arrow::Top | Arrow::Bottom => {
                    self.zoom_property_context(
                        mouse_over_row.property_name,
                        match arrow {
                            Arrow::Top => 10.0,
                            Arrow::Bottom => -10.0,
                            _ => unreachable!(),
                        },
                        mouse_over_row.mouse_local_xy.y,
                        row_height,
                    );
                }
            }
        } else if namui::system::keyboard::any_code_press([Code::ShiftLeft, Code::ShiftRight]) {
            match arrow {
                Arrow::Left | Arrow::Right => {
                    self.context.start_at += self.context.time_per_pixel
                        * PixelSize::from(match arrow {
                            Arrow::Left => -10.0,
                            Arrow::Right => 10.0,
                            _ => unreachable!(),
                        });
                }
                Arrow::Top | Arrow::Bottom => {
                    self.move_property_context_by(
                        mouse_over_row.property_name,
                        match arrow {
                            Arrow::Top => 10.0,
                            Arrow::Bottom => -10.0,
                            _ => unreachable!(),
                        },
                    );
                }
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
        namui::event::send(crate::Event::UpdateLayer(Arc::new(layer)));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arrow {
    Left,
    Right,
    Top,
    Bottom,
}
