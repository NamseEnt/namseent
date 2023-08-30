use namui::prelude::*;
use namui_prebuilt::{simple_rect, table, typography::text_fit};
use std::ops::Deref;

static NOTIFICATIONS_ATOM: Atom<Vec<Notification>> = Atom::uninitialized_new();
fn atom(ctx: &RenderCtx) -> (Sig<Vec<Notification>>, SetState<Vec<Notification>>) {
    ctx.atom_init(&NOTIFICATIONS_ATOM, Vec::new)
}

#[component]
pub struct NotificationRoot {
    pub wh: Wh<Px>,
}
impl Component for NotificationRoot {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
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
            table::hooks::vertical_padding(PADDING, |wh, ctx| {
                table::hooks::vertical(notifications.deref().iter().map(|notification| {
                    table::hooks::fixed(NOTIFICATION_CARD_HEIGHT, |wh, ctx| {
                        table::hooks::padding(PADDING, |wh, ctx| {
                            table::hooks::horizontal([
                                table::hooks::fixed(wh.height, |wh, ctx| {
                                    if notification.loading {
                                        ctx.add(LoadingIndicator {
                                            wh,
                                            color: notification.level.text_color(),
                                        });
                                    }
                                }),
                                table::hooks::ratio(1, |wh, ctx| {
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
                            ])(wh, ctx);
                            ctx.add(simple_rect(
                                wh,
                                notification.level.text_color(),
                                2.px(),
                                notification.level.background_color(),
                            ));
                        })(wh, ctx)
                    })
                }))(wh, ctx)
            })(wh, &mut ctx);
        });

        ctx.done()
    }
}

#[derive(Debug)]
pub struct Notification {
    id: Uuid,
    level: NotificationLevel,
    message: String,
    loading: bool,
}
impl Notification {
    fn new(level: NotificationLevel, message: String) -> Self {
        Self {
            id: uuid(),
            level,
            message,
            loading: false,
        }
    }
    pub fn set_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
    pub fn info(message: String) -> Self {
        Self::new(NotificationLevel::Info, message)
    }
}

#[derive(Debug)]
enum NotificationLevel {
    Info,
}
impl NotificationLevel {
    fn text_color(&self) -> Color {
        match self {
            Self::Info => Color::WHITE,
        }
    }
    fn background_color(&self) -> Color {
        match self {
            Self::Info => Color::from_u8(68, 170, 238, 255),
        }
    }
}

#[component]
struct LoadingIndicator {
    wh: Wh<Px>,
    color: Color,
}
impl Component for LoadingIndicator {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, color } = self;
        let stroke_width = wh.height / 8.0;
        let wh = Wh {
            width: wh.width - stroke_width * 2.0,
            height: wh.height - stroke_width * 2.0,
        };
        let xy = Xy::single(stroke_width);
        let now = now();
        let start_angle = Angle::Degree(360.0 * (now.as_seconds() % 1.0));
        let delta_angle = Angle::Radian(now.as_seconds().sin() * 3.0);
        let path = Path::new().add_arc(Rect::from_xy_wh(Xy::zero(), wh), start_angle, delta_angle);
        let paint = Paint::new()
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(wh.height / 8.0)
            .set_color(color);
        ctx.compose(|ctx| {
            ctx.translate(xy).add(namui::path(path, paint));
        });

        ctx.done()
    }
}

pub fn push_notification(notification: Notification) -> Uuid {
    let id = notification.id;
    NOTIFICATIONS_ATOM.mutate(|notifications| {
        notifications.push(notification);
    });
    id
}

pub fn remove_notification(id: Uuid) {
    NOTIFICATIONS_ATOM.mutate(move |notifications| {
        if let Some(index) = notifications
            .iter()
            .position(|notification| notification.id == id)
        {
            notifications.remove(index);
        }
    })
}
