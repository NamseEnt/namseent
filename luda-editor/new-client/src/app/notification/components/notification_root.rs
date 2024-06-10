use super::*;
use namui::*;
use namui_prebuilt::{simple_rect, typography::text_fit};
use std::ops::Deref;

pub struct NotificationRoot {
    pub wh: Wh<Px>,
}
impl Component for NotificationRoot {
    fn render(self, ctx: &RenderCtx)  {
        const NOTIFICATION_CARD_HEIGHT: Px = px(48.0);
        const PADDING: Px = px(8.0);
        const MAX_WIDTH: Px = px(720.0);

        let Self { wh } = self;
        let xy = Xy {
            x: ((wh.width - MAX_WIDTH) / 2.0).max(0.px()),
            y: 0.px(),
        };
        let wh = Wh {
            width: wh.width.min(MAX_WIDTH),
            height: wh.height,
        };

        let (notifications, _set_notifications) = atom(ctx);

        ctx.compose(|ctx| {
            let mut ctx = ctx.translate(xy);
            namui_prebuilt::table::hooks::vertical_padding(PADDING, |wh, ctx| {
                namui_prebuilt::table::hooks::vertical(notifications.deref().iter().map(
                    |notification| {
                        namui_prebuilt::table::hooks::fixed(NOTIFICATION_CARD_HEIGHT, |wh, ctx| {
                            namui_prebuilt::table::hooks::padding(PADDING, |wh, ctx| {
                                namui_prebuilt::table::hooks::horizontal([
                                    namui_prebuilt::table::hooks::fixed(wh.height, |wh, ctx| {
                                        if notification.loading {
                                            ctx.add(LoadingIndicator {
                                                wh,
                                                color: notification.level.text_color(),
                                            });
                                        }
                                    }),
                                    namui_prebuilt::table::hooks::ratio(1, |wh, ctx| {
                                        ctx.clip(
                                            Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), wh)),
                                            ClipOp::Intersect,
                                        )
                                        .add(text_fit(
                                            wh.height,
                                            notification.message.clone(),
                                            notification.level.text_color(),
                                            PADDING,
                                        ));
                                    }),
                                    namui_prebuilt::table::hooks::fixed(wh.height, |wh, ctx| {
                                        ctx.add(CopyButton {
                                            wh,
                                            color: notification.level.text_color(),
                                            content: &notification.message,
                                        });
                                    }),
                                    namui_prebuilt::table::hooks::fixed(wh.height, |wh, ctx| {
                                        if !notification.loading {
                                            ctx.add(
                                                CloseButton {
                                                    wh,
                                                    color: notification.level.text_color(),
                                                }
                                                .attach_event(|event| {
                                                    if let Event::MouseDown { event } = event {
                                                        if event.is_local_xy_in() {
                                                            event.stop_propagation();
                                                            remove_notification(notification.id);
                                                        }
                                                    }
                                                })
                                                .with_mouse_cursor(MouseCursor::Pointer),
                                            );
                                        }
                                    }),
                                ])(wh, ctx);
                                ctx.add(simple_rect(
                                    wh,
                                    notification.level.text_color(),
                                    2.px(),
                                    notification.level.background_color(),
                                ));
                            })(wh, ctx)
                        })
                    },
                ))(wh, ctx)
            })(wh, &mut ctx);
        });

        
    }
}
