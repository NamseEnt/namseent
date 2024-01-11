mod app;

use app::App;

pub fn main() {
    namui::start(|| App {})
}
