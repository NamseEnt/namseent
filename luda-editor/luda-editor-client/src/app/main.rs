use super::app::App;

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut App::new(), &()).await
}
