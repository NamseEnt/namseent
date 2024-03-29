use crate::app::notification::{self};
use namui::*;
use namui_prebuilt::simple_rect;

#[component]
pub struct CopyButton<'a> {
    pub wh: Wh<Px>,
    pub color: Color,
    pub content: &'a str,
}
impl Component for CopyButton<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh, color, content } = self;
        let (copied, set_copied) = ctx.state(|| false);
        let color = match *copied {
            true => color.brighter(0.3),
            false => color,
        };
        ctx.compose(|ctx| {
            namui_prebuilt::table::hooks::padding(wh.height / 6.0, |wh, ctx| {
                let stroke_width = wh.height / 8.0;
                let offset = wh / 4.0;
                let wh = wh - offset;
                let front_rect = Rect::Xywh {
                    x: 0.px(),
                    y: offset.height,
                    width: wh.width,
                    height: wh.height,
                };
                let rear_rect = Rect::Xywh {
                    x: offset.width,
                    y: 0.px(),
                    width: wh.width,
                    height: wh.height,
                };
                let rect_style = RectStyle {
                    stroke: Some(RectStroke {
                        color,
                        width: stroke_width,
                        border_position: BorderPosition::Middle,
                    }),
                    fill: Some(RectFill {
                        color: Color::TRANSPARENT,
                    }),
                    round: Some(RectRound {
                        radius: stroke_width,
                    }),
                };
                ctx.add(rect(RectParam {
                    rect: front_rect,
                    style: rect_style,
                }));
                ctx.clip(Path::new().add_rect(front_rect), ClipOp::Difference)
                    .add(rect(RectParam {
                        rect: rear_rect,
                        style: rect_style,
                    }));

                ctx.add(
                    simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT)
                        .attach_event(|event| match event {
                            Event::MouseDown { event } => {
                                if !event.is_local_xy_in() {
                                    return;
                                }
                                event.stop_propagation();
                                let content = content.to_string();
                                spawn_local(async move {
                                    let Err(error) = clipboard::write_text(content).await else {
                                        set_copied.set(true);
                                        return;
                                    };
                                    notification::error!("{error}").push();
                                })
                            }
                            Event::MouseUp { event } => {
                                if event.is_local_xy_in() {
                                    event.stop_propagation();
                                }
                            }
                            _ => {}
                        })
                        .with_mouse_cursor(MouseCursor::Pointer),
                );
            })(wh, ctx);
        });

        
    }
}
