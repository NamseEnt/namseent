use super::*;
use rpc::data::Circumscribed;

#[namui::component]
pub struct Mover<'a> {
    pub image_dest_rect: Rect<Px>,
    pub dragging: Option<Dragging>,
    pub container_wh: Wh<Px>,
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

pub enum Event {
    MoveStart {
        start_global_xy: Xy<Px>,
        end_global_xy: Xy<Px>,
        container_wh: Wh<Px>,
    },
}

impl Component for Mover<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            image_dest_rect,
            ref dragging,
            container_wh,
            ref on_event,
        } = self;
        let on_event = on_event.clone();

        ctx.compose(|ctx| {
            ctx.translate((image_dest_rect.x(), image_dest_rect.y()))
                .add(
                    simple_rect(
                        Wh {
                            width: image_dest_rect.width(),
                            height: image_dest_rect.height(),
                        },
                        Color::grayscale_f01(0.2),
                        px(2.0),
                        Color::TRANSPARENT,
                    )
                    .with_mouse_cursor({
                        let is_dragging = matches!(dragging, Some(Dragging::Mover { .. }));
                        if is_dragging {
                            namui::MouseCursor::Move
                        } else {
                            namui::MouseCursor::Pointer
                        }
                    })
                    .attach_event(|event| {
                        if let namui::Event::MouseDown { event } = event {
                            if event.is_local_xy_in() && event.button == Some(MouseButton::Left) {
                                event.stop_propagation();
                                on_event(Event::MoveStart {
                                    start_global_xy: event.global_xy,
                                    end_global_xy: event.global_xy,
                                    container_wh,
                                });
                            }
                        }
                    }),
                );
        });

        ctx.done()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoverDraggingContext {
    pub start_global_xy: Xy<Px>,
    pub end_global_xy: Xy<Px>,
    pub container_wh: Wh<Px>,
}

impl MoverDraggingContext {
    pub fn move_circumscribed(
        &self,
        circumscribed: Circumscribed<Percent>,
    ) -> Circumscribed<Percent> {
        let delta_xy = self.end_global_xy - self.start_global_xy;
        let delta_xy_percent: Xy<Percent> = Xy::new(
            (delta_xy.x / self.container_wh.width).into(),
            (delta_xy.y / self.container_wh.height).into(),
        );
        Circumscribed {
            center_xy: circumscribed.center_xy + delta_xy_percent,
            ..circumscribed
        }
    }
}
