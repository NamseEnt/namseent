use namui::prelude::*;
use namui_animation_editor::v1_5::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn start() {
    let namui_context = namui::init();

    namui::start(namui_context, &mut AnimationEditorExample::new(), &()).await
}

struct AnimationEditorExample {}

impl AnimationEditorExample {
    fn new() -> Self {
        Self {}
    }
}

impl Entity for AnimationEditorExample {
    type Props = ();

    fn render(&self, props: &Self::Props) -> RenderingTree {}

    fn update(&mut self, event: &dyn std::any::Any) {
        // if let Some(event) = event.downcast_ref::<_>() {
        // }
    }
}
