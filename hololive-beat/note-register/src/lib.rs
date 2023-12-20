mod note;

use namui::prelude::*;
use note::load_notes;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub async fn main() {
    let namui_context = namui::init().await;

    namui_context.start(|| Init {}).await;
}

#[namui::component]
struct Init {}

impl namui::Component for Init {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (loaded, set_loaded) = ctx.state(|| false);

        ctx.effect("Init", || {
            spawn_local(async move {
                let notes = load_notes().await;
                namui::log!("{notes:#?}");
                set_loaded.set(true);
            })
        });

        // ctx.component(loaded.then(|| app::App {}));
        ctx.done()
    }
}
