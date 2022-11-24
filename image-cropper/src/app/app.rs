use super::router::{Router, RouterProps};
use namui::{Entity, RenderingTree, Wh};

pub struct App {
    router: Router,
}

impl Entity for App {
    type Props = ();

    fn update(&mut self, event: &namui::Event) {
        self.router.update(event);
    }

    fn render(&self, _: &Self::Props) -> RenderingTree {
        let screen_size = namui::screen::size();
        self.router.render(&RouterProps {
            screen_wh: Wh {
                width: screen_size.width,
                height: screen_size.height,
            },
        })
    }
}
impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }
}
