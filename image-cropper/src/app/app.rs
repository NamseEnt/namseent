use super::router::{Router, RouterProps};
use namui::{Entity, RenderingTree, Wh};

pub struct App {
    router: Router,
}

impl Entity for App {
    type Props = ();

    fn update(&mut self, event: &dyn std::any::Any) {
        self.router.update(event);
    }

    fn render(&self, _: &Self::Props) -> RenderingTree {
        let screen_size = namui::system::screen::size();
        self.router.render(&RouterProps {
            screen_wh: Wh {
                width: screen_size.width as f32,
                height: screen_size.height as f32,
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
