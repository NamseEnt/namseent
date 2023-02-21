mod login;

use crate::pages::router;
use namui::prelude::*;
use namui_prebuilt::*;

pub enum App {
    LoggingIn,
    LoggedIn { router: router::Router },
}
impl App {
    pub fn new() -> Self {
        login::check_session_id();
        App::LoggingIn
    }
}

enum Event {
    LoggedIn,
}

impl namui::Entity for App {
    type Props = ();

    fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| match event {
            Event::LoggedIn => {
                *self = App::LoggedIn {
                    router: router::Router::new(),
                };
            }
        });

        self.update_login(event);

        match self {
            App::LoggingIn => {}
            App::LoggedIn { router, .. } => {
                router.update(event);
            }
        }
    }
    fn render(&self, _: &Self::Props) -> namui::RenderingTree {
        let wh = namui::screen::size();
        render([
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::BLACK),
            match &self {
                App::LoggingIn => typography::body::center(wh, "Logging in...", Color::BLACK),
                App::LoggedIn { router } => router.render(router::Props { wh }),
            },
        ])
    }
}
