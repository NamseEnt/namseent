use namui::prelude::*;
use namui_prebuilt::{simple_rect, typography};

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, || MouseExample).await
}

#[namui::component]
struct MouseExample;

impl Component for MouseExample {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (text, set_text) = ctx.state(|| "".to_string());
        ctx.component(typography::body::left_top(
            text.as_ref().clone(),
            Color::BLACK,
        ));

        ctx.component(
            simple_rect(
                Wh::new(200.px(), 200.px()),
                Color::BLACK,
                1.0.px(),
                Color::WHITE,
            )
            .attach_event(|event| match event {
                Event::MouseDown { event } => {
                    set_text.set(format!("Mouse Down: {:?}", event));
                }
                Event::MouseMove { event } => {
                    set_text.set(format!("Mouse Move: {:?}", event));
                }
                Event::MouseUp { event } => {
                    set_text.set(format!("Mouse Up: {:?}", event));
                }
                _ => {}
            }),
        );

        ctx.done()
    }
}
