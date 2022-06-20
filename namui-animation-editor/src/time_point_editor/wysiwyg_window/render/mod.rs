use super::*;
mod viewport;

impl WysiwygWindow {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let animation = self.animation.read();

        clip(
            PathBuilder::new().add_rect(&LtrbRect {
                left: 0.0,
                top: 0.0,
                right: props.wh.width.into(),
                bottom: props.wh.height.into(),
            }),
            ClipOp::Intersect,
            render([
                self.render_viewport(),
                simple_rect(props.wh, Color::BLACK, 1.0, Color::TRANSPARENT),
            ])
            .attach_event(|builder| {
                builder
                    .on_mouse_down(|event| {
                        namui::event::send(super::Event::BackgroundClicked {
                            mouse_xy: event.local_xy,
                        });
                    })
                    .on_mouse_move_in(|event| {
                        namui::event::send(super::Event::MouseMoveIn {
                            mouse_xy: event.local_xy,
                        });
                    })
            }),
        )
    }
}
