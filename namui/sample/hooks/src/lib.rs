#![feature(fn_traits)]
#![feature(unboxed_closures)]

mod hooks;

use hooks::Hooks;
use namui::prelude::*;

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut HooksExample::new(), &()).await
}

struct HooksExample {
    hooks: Hooks,
}

impl HooksExample {
    fn new() -> Self {
        Self {
            hooks: Hooks::new(),
        }
    }
}

impl Entity for HooksExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        translate(100.px(), 100.px(), self.hooks.render())
    }

    fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|_event| {});
    }
}
