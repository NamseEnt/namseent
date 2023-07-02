use super::*;
mod layer;
mod viewport;

impl WysiwygWindow {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let animation = props.animation;
        let wh = props.wh;

        let layers = animation
            .layers
            .iter()
            .map(|layer| self.render_layer(&props, layer));

        let background = simple_rect(props.wh, Color::BLACK, px(1.0), Color::TRANSPARENT)
            .with_id(self.window_id)
            .attach_event(|builder| {
                builder
                    .on_mouse_down_in(|event: MouseEvent| {
                        namui::event::send(super::Event::BackgroundClicked {
                            mouse_xy: event.local_xy,
                        });
                    })
                    .on_mouse_move_in(|event: MouseEvent| {
                        namui::event::send(super::Event::MouseMoveIn {
                            mouse_local_xy: event.local_xy,
                        });
                    })
                    .on_mouse(|_| {
                        namui::event::send(super::Event::MouseUp);
                    })
                    .on_wheel(move |event: WheelEvent| {
                        if namui::keyboard::any_code_press([Code::ShiftLeft, Code::ShiftRight]) {
                            namui::event::send(super::Event::ShiftWheel {
                                delta: event.delta_xy.y,
                            });
                        } else if namui::keyboard::any_code_press([Code::AltLeft, Code::AltRight]) {
                            namui::event::send(super::Event::AltWheel {
                                delta: event.delta_xy.y,
                                mouse_local_xy: event.mouse_local_xy,
                            });
                        } else {
                            namui::event::send(super::Event::Wheel {
                                delta: event.delta_xy.y,
                            });
                        }
                    })
                    .on_key_down(move |event: KeyboardEvent| {
                        if event.code == Code::Home {
                            namui::event::send(super::Event::HomeKeyDown { wh });
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
