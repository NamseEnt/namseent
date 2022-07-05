use super::*;

impl GraphWindow {
    pub(super) fn handle_key_down(&mut self, code: Code, row_height: f32) {
        self.handle_arrow_key_down(code, row_height);
    }

    fn handle_arrow_key_down(&mut self, code: Code, row_height: f32) {
        let arrow = match code {
            Code::ArrowLeft => Arrow::Left,
            Code::ArrowRight => Arrow::Right,
            Code::ArrowUp => Arrow::Up,
            Code::ArrowDown => Arrow::Down,
            _ => return,
        };
        self.handle_point_move(arrow);
        self.handle_graph_move_and_zoom(arrow, row_height);
    }

    fn handle_point_move(&mut self, arrow: Arrow) {
        if self.selected_point_address.is_none() {
            return;
        }
        let selected_point_address = self.selected_point_address.as_ref().unwrap();

        let delta_y = match arrow {
            Arrow::Up => Px::from_f32(1.0).unwrap(),
            Arrow::Down => Px::from_f32(-1.0).unwrap(),
            _ => return,
        };

        if let Some(ticket) = self
            .animation_history
            .try_set_action(self.get_move_by_action(selected_point_address, delta_y))
        {
            self.animation_history.act(ticket).unwrap();
        }
    }

    fn handle_graph_move_and_zoom(&mut self, arrow: Arrow, row_height: f32) {
        if self.mouse_over_row.is_none() {
            return;
        }
        let mouse_over_row = self.mouse_over_row.clone().unwrap();

        if namui::keyboard::any_code_press([Code::AltLeft, Code::AltRight]) {
            match arrow {
                Arrow::Left | Arrow::Right => {
                    let time_at_mouse_position = self.context.start_at
                        + Px::from(mouse_over_row.mouse_xy_in_row.x) * self.context.time_per_px;

                    let next_time_per_px = zoom_time_per_px(
                        self.context.time_per_px,
                        match arrow {
                            Arrow::Left => 10.0,
                            Arrow::Right => -10.0,
                            _ => unreachable!(),
                        },
                    );

                    let next_start_at = time_at_mouse_position
                        - Px::from(mouse_over_row.mouse_xy_in_row.x) * next_time_per_px;

                    self.context.time_per_px = next_time_per_px;
                    self.context.start_at = next_start_at;
                }
                Arrow::Up | Arrow::Down => {
                    self.zoom_property_context(
                        mouse_over_row.property_name,
                        match arrow {
                            Arrow::Up => 10.0,
                            Arrow::Down => -10.0,
                            _ => unreachable!(),
                        },
                        mouse_over_row.mouse_xy_in_row.y,
                        row_height,
                    );
                }
            }
        } else if namui::keyboard::any_code_press([Code::ShiftLeft, Code::ShiftRight]) {
            match arrow {
                Arrow::Left | Arrow::Right => {
                    self.context.start_at += self.context.time_per_px
                        * Px::from(match arrow {
                            Arrow::Left => -10.0,
                            Arrow::Right => 10.0,
                            _ => unreachable!(),
                        });
                }
                Arrow::Up | Arrow::Down => {
                    self.move_property_context_by(
                        mouse_over_row.property_name,
                        match arrow {
                            Arrow::Up => 10.0,
                            Arrow::Down => -10.0,
                            _ => unreachable!(),
                        },
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arrow {
    Left,
    Right,
    Up,
    Down,
}
