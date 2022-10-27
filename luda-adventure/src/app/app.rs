use super::game::Game;

pub struct App {
    game: Game,
}
impl App {
    pub fn new() -> Self {
        Self {
            game: Game::new_with_mock(),
        }
    }
}

impl namui::Entity for App {
    type Props = ();

    fn update(&mut self, event: &dyn std::any::Any) {
        self.game.update(event);
    }

    fn render(&self, _: &Self::Props) -> namui::RenderingTree {
        self.game.render()
    }
}
