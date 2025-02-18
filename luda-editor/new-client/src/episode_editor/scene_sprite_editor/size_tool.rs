use std::ops::Deref;

use crate::*;

pub static SIZE_TOOL_DRAGGING_ATOM: Atom<Option<SizeToolDragging>> = Atom::uninitialized();

pub struct SizeTool<'a> {
    pub wh: Wh<Px>,
    pub size_radius: Percent,
    /// Fn(Percent, is_dragging)
    pub on_change_size_radius: &'a dyn Fn(Percent, bool),
}

impl Component for SizeTool<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            size_radius,
            on_change_size_radius,
        } = self;

        let (dragging, set_dragging) = ctx.init_atom(&SIZE_TOOL_DRAGGING_ATOM, || None);

        let size_radius = dragging
            .deref()
            .as_ref()
            .map(|dragging| dragging.radius)
            .unwrap_or(size_radius);

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(64.px(), |wh, ctx| {
                    ctx.add(typography::body::left(
                        wh.height,
                        format!("크기 - {}", size_radius),
                        Color::WHITE,
                    ));
                }),
                table::ratio(1, |wh, ctx| {
                    ctx.add(
                        rect(RectParam {
                            rect: Rect::zero_wh(wh),
                            style: RectStyle {
                                stroke: Some(RectStroke {
                                    color: Color::WHITE,
                                    width: 1.px(),
                                    border_position: BorderPosition::Inside,
                                }),
                                fill: None,
                                round: None,
                            },
                        })
                        .attach_event(|event| {
                            let mouse_event = match &event {
                                Event::MouseDown { event } => {
                                    if !event.is_local_xy_in() {
                                        return;
                                    }
                                    event
                                }
                                Event::MouseMove { event } | Event::MouseUp { event } => {
                                    if dragging.is_none() {
                                        return;
                                    }
                                    event
                                }
                                Event::VisibilityChange => {
                                    if dragging.is_some() {
                                        set_dragging.set(None);
                                    }
                                    return;
                                }
                                _ => {
                                    return;
                                }
                            };
                            let radius = {
                                let x = (mouse_event.local_xy().x / wh.width).clamp(0.0, 1.0);
                                100.percent() * x
                            };
                            on_change_size_radius(radius, true);

                            if let Event::MouseUp { .. } = &event {
                                set_dragging.set(None);
                                on_change_size_radius(radius, false);
                            }
                        }),
                    );

                    let cursor_width = 24.px();

                    let x = wh.width * size_radius;
                    ctx.translate((x, 0.px())).add(path(
                        namui::Path::new().add_rect(Rect::from_xy_wh(
                            Xy::new(-cursor_width / 2, 0.px()),
                            Wh::new(cursor_width, wh.height),
                        )),
                        Paint::new(Color::WHITE).set_style(PaintStyle::Stroke),
                    ));
                }),
            ])(wh, ctx)
        });
    }
}

pub struct SizeToolDragging {
    pub scene_id: u128,
    pub sprite_index: usize,
    pub radius: Percent,
}
