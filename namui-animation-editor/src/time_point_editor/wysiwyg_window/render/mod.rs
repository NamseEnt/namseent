use super::*;
mod layer;
mod viewport;

impl WysiwygWindow {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let animation = props.animation;
        let wh = props.wh;

        if self.last_wh != Some(wh) {
            namui::event::send(Event::UpdateWh { wh });
            return RenderingTree::Empty;
        }

        let layers = animation
            .layers
            .iter()
            .map(|layer| self.render_layer(&props, layer));

        let background = simple_rect(props.wh, Color::BLACK, px(1.0), Color::TRANSPARENT)
            .with_id(&self.window_id)
            .attach_event(|builder| {
                builder
                    .on_mouse_down_in(|event| {
                        namui::event::send(super::Event::BackgroundClicked {
                            mouse_xy: event.local_xy,
                        });
                    })
                    .on_mouse_move_in(|event| {
                        namui::event::send(super::Event::MouseMoveIn {
                            mouse_local_xy: event.local_xy,
                        });
                    })
                    .on_wheel(move |event| {
                        let mouse_global_xy = namui::mouse::position();
                        let row_xy = event
                            .namui_context
                            .get_rendering_tree_xy(event.target)
                            .expect("ERROR: fail to get rendering_tree_xy");

                        let mouse_local_xy = mouse_global_xy - row_xy;

                        if mouse_local_xy.x < px(0.0)
                            || wh.width < mouse_local_xy.x
                            || mouse_local_xy.y < px(0.0)
                            || wh.height < mouse_local_xy.y
                        {
                            return;
                        }
                        if namui::keyboard::any_code_press([Code::ShiftLeft, Code::ShiftRight]) {
                            namui::event::send(super::Event::ShiftWheel {
                                delta: event.delta_xy.y,
                            });
                        } else if namui::keyboard::any_code_press([Code::AltLeft, Code::AltRight]) {
                            namui::event::send(super::Event::AltWheel {
                                delta: event.delta_xy.y,
                                mouse_local_xy,
                            });
                        } else {
                            namui::event::send(super::Event::Wheel {
                                delta: event.delta_xy.y,
                            });
                        }
                    });
            });

        clip(
            PathBuilder::new().add_rect(Rect::from_xy_wh(Xy::single(px(0.0)), props.wh)),
            ClipOp::Intersect,
            render([
                background,
                scale(
                    1.0 / self.real_px_per_screen_px,
                    1.0 / self.real_px_per_screen_px,
                    translate(
                        -self.real_left_top_xy.x,
                        -self.real_left_top_xy.y,
                        render(layers.chain([self.render_viewport()])),
                    ),
                ),
            ]),
        )
    }
}
