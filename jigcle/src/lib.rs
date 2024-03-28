// mod app;

// use app::App;

mod game;

pub fn main() {
    namui::start(|| game::Game {})
}
