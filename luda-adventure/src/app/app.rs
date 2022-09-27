use super::game::Game;

pub struct App<'a> {
    game: Game<'a>,
}
impl App<'_> {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }
}

impl namui::Entity for App<'_> {
    type Props = ();

    fn update(&mut self, event: &dyn std::any::Any) {
        self.game.update(event);
    }

    fn render(&self, _: &Self::Props) -> namui::RenderingTree {
        self.game.render()
    }
}
