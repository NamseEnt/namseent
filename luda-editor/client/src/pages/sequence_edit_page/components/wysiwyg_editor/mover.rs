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
        moving_with: MovingWith,
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
                        match *dragging {
                            Some(Dragging::Mover { context })
                                if context.moving_with == MovingWith::Mouse =>
                            {
                                namui::MouseCursor::Move
                            }
                            _ => namui::MouseCursor::Pointer,
                        }
                    })
                    .attach_event(|event| {
                        if let namui::Event::MouseDown { event } = event {
                            if event.is_local_xy_in()
                                && dragging.is_none()
                                && event.button == Some(MouseButton::Left)
                            {
                                event.stop_propagation();
                                on_event(Event::MoveStart {
                                    start_global_xy: event.global_xy,
                                    end_global_xy: event.global_xy,
                                    container_wh,
                                    moving_with: MovingWith::Mouse,
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
    pub moving_with: MovingWith,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MovingWith {
    Mouse,
    KeyLeft,
    KeyRight,
    KeyUp,
    KeyDown,
}
impl MovingWith {
    pub fn key_changed(&self, code: namui::Code) -> bool {
        match self {
            MovingWith::Mouse => true,
            MovingWith::KeyLeft => code != namui::Code::ArrowLeft,
            MovingWith::KeyRight => code != namui::Code::ArrowRight,
            MovingWith::KeyUp => code != namui::Code::ArrowUp,
            MovingWith::KeyDown => code != namui::Code::ArrowDown,
        }
    }
    pub fn delta_xy(&self) -> Xy<Px> {
        match self {
            MovingWith::Mouse => Xy::new(0.0.px(), 0.0.px()),
            MovingWith::KeyLeft => Xy::new(-(1.0.px()), 0.0.px()),
            MovingWith::KeyRight => Xy::new(1.0.px(), 0.0.px()),
            MovingWith::KeyUp => Xy::new(0.0.px(), -(1.0.px())),
            MovingWith::KeyDown => Xy::new(0.0.px(), 1.0.px()),
        }
    }
}
impl TryFrom<namui::Code> for MovingWith {
    type Error = ();
    fn try_from(code: namui::Code) -> Result<Self, Self::Error> {
        match code {
            namui::Code::ArrowLeft => Ok(MovingWith::KeyLeft),
            namui::Code::ArrowRight => Ok(MovingWith::KeyRight),
            namui::Code::ArrowUp => Ok(MovingWith::KeyUp),
            namui::Code::ArrowDown => Ok(MovingWith::KeyDown),
            _ => Err(()),
        }
    }
}
