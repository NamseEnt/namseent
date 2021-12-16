use super::*;
use ::namui::*;

struct Main {
    editor: Editor,
}

impl namui::Entity for Main {
    type Props = ();
    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        self.editor.render(&())
    }
    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<namui::NamuiEvent>() {
            match event {
                &namui::NamuiEvent::ScreenResize(wh) => {
                    self.editor.resize(namui::Wh {
                        width: wh.width as f32,
                        height: wh.height as f32,
                    });
                }
                _ => {}
            }
        }

        self.editor.update(event);
    }
}

pub async fn main() {
    let namui_context = namui::init();
    let screen_size = namui::screen::size();

    namui::start(
        namui_context,
        &mut Main {
            editor: Editor::new(namui::Wh {
                width: screen_size.width as f32,
                height: screen_size.height as f32,
            }),
        },
        &(),
    )
    .await
}
