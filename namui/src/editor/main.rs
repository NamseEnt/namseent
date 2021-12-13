use super::*;
use crate::engine;

struct Main {
    editor: Editor,
}

impl engine::Entity for Main {
    type Props = ();
    fn render(&self, props: &Self::Props) -> engine::RenderingTree {
        self.editor.render(&())
    }
    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<engine::EngineEvent>() {
            match event {
                &engine::EngineEvent::ScreenResize(wh) => {
                    self.editor.resize(engine::Wh {
                        width: wh.width as f32,
                        height: wh.height as f32,
                    });
                }
                _ => {}
            }
        }
    }
}

pub async fn main() {
    let engine_context = engine::init();
    let screen_size = engine::screen::size();

    engine::start(
        engine_context,
        &mut Main {
            editor: Editor::new(engine::Wh {
                width: screen_size.width as f32,
                height: screen_size.height as f32,
            }),
        },
        &(),
    )
    .await
}
