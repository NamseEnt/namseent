use super::*;
use namui::prelude::*;

struct Main {
    editor: Editor,
}

impl namui::Entity for Main {
    type Props = ();
    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        self.editor.render(&())
    }
    fn update(&mut self, event: &dyn std::any::Any) {
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
