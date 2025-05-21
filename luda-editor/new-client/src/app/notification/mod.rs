mod components;

pub use components::NotificationRoot;
use namui::*;

static NOTIFICATIONS_ATOM: Atom<Vec<Notification>> = Atom::uninitialized_new();
fn atom(ctx: &RenderCtx) -> (Sig<Vec<Notification>>, SetState<Vec<Notification>>) {
    ctx.atom_init(&NOTIFICATIONS_ATOM, Vec::new)
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
    pub fn error(message: String) -> Self {
        Self::new(NotificationLevel::Error, message)
    }
    pub fn push(self) -> Uuid {
        push_notification(self)
    }
}

#[derive(Debug)]
enum NotificationLevel {
    Info,
    Error,
}
impl NotificationLevel {
    fn text_color(&self) -> Color {
        match self {
            NotificationLevel::Info => Color::WHITE,
            NotificationLevel::Error => Color::WHITE,
        }
    }
    fn background_color(&self) -> Color {
        match self {
            NotificationLevel::Info => Color::from_u8(68, 170, 238, 255),
            NotificationLevel::Error => Color::from_u8(255, 51, 102, 255),
        }
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

macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::app::notification::Notification::info(format!($($arg)*))
    }}
}
pub(crate) use info;

macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::app::notification::Notification::error(format!($($arg)*))
    }}
}
pub(crate) use error;
