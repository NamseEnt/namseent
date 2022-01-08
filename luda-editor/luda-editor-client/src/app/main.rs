use super::app::App;

pub async fn main() {
    let namui_context = namui::init();

    namui::start(namui_context, &mut App::new(), &()).await
}
