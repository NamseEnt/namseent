use namui::*;

pub struct Rotator<'a> {
    pub rect: Rect<Px>,
    pub dragging_context: Option<RotatorDraggingContext>,
    pub graphic_index: Uuid,
    pub on_event: &'a dyn Fn(Event),
}

pub enum Event {
    OnRotate {
        rotation: Angle,
        graphic_index: Uuid,
    },
    OnUpdateDraggingContext {
        context: Option<RotatorDraggingContext>,
    },
}

impl Component for Rotator<'_> {
    fn render(self, ctx: &RenderCtx)  {
        const HANDLE_RADIUS: Px = px(5.0);
        const HANDLE_OFFSET: Px = px(18.0);
        let Self {
            rect,
            dragging_context,
            graphic_index,
            on_event,
        } = self;

        let center_x = rect.center().x;
        let top_y = rect.y();
        let handle_xy = Xy {
            x: center_x,
            y: top_y - HANDLE_OFFSET,
        };
        let handle_path = namui::Path::new().add_oval(Rect::Ltrb {
            left: handle_xy.x - HANDLE_RADIUS,
            top: handle_xy.y - HANDLE_RADIUS,
            right: handle_xy.x + HANDLE_RADIUS,
            bottom: handle_xy.y + HANDLE_RADIUS,
        });

        let bridge_path = namui::Path::new()
            .move_to(center_x, top_y)
            .line_to(center_x, top_y - HANDLE_OFFSET);

        let fill_paint = namui::Paint::new()
            .set_style(namui::PaintStyle::Fill)
            .set_color(Color::WHITE);

        let stroke_paint = namui::Paint::new()
            .set_style(namui::PaintStyle::Stroke)
            .set_color(Color::grayscale_f01(0.5))
            .set_stroke_width(px(2.0))
            .set_anti_alias(true);

        let rendering_tree = render([
            namui::path(handle_path.clone(), fill_paint),
            namui::path(handle_path, stroke_paint.clone()),
            namui::path(bridge_path, stroke_paint),
        ])
        .with_mouse_cursor(MouseCursor::Crosshair)
        .attach_event(move |event| match dragging_context {
            Some(context) => match event {
                namui::Event::MouseMove { event } => {
                    on_event(Event::OnUpdateDraggingContext {
                        context: Some(RotatorDraggingContext {
                            origin_global_xy: context.origin_global_xy,
                            end_global_xy: event.global_xy,
                        }),
                    });
                }
                namui::Event::MouseUp { event: _event } => {
                    on_event(Event::OnUpdateDraggingContext { context: None });
                    on_event(Event::OnRotate {
                        rotation: context.rotation(keyboard::shift_press()),
                        graphic_index,
                    });
                }
                _ => {}
            },
            None => {
                if let namui::Event::MouseDown { event } = event {
                    if event.is_local_xy_in() && event.button == Some(MouseButton::Left) {
                        event.stop_propagation();
                        on_event(Event::OnUpdateDraggingContext {
                            context: Some(RotatorDraggingContext {
                                origin_global_xy: ctx.global_xy(rect.center()),
                                end_global_xy: event.global_xy,
                            }),
                        });
                    }
                }
            }
        });

        ctx.component(rendering_tree);

        
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotatorDraggingContext {
    origin_global_xy: Xy<Px>,
    end_global_xy: Xy<Px>,
}

impl RotatorDraggingContext {
    pub fn rotation(&self, snap: bool) -> Angle {
        const SNAP_ANGLE_DEGREE: f32 = 15.0;
        let angle =
            Xy::new(0.0.px(), -(1.0.px())).angle_to(self.end_global_xy - self.origin_global_xy);

        if !snap {
            return angle;
        }
        Angle::Degree((angle.as_degrees() / SNAP_ANGLE_DEGREE).round() * SNAP_ANGLE_DEGREE)
    }
}
