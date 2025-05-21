use namui::*;
use namui_prebuilt::{simple_rect, typography};

pub fn main() {
    namui::start(|ctx| {
        ctx.add(MouseExample);
    })
}

struct MouseExample;

impl Component for MouseExample {
    fn render(self, ctx: &RenderCtx) {
        let (text, set_text) = ctx.state(|| {
            println!("Init state");
            "".to_string()
        });
        ctx.add(typography::body::left_top(
            text.as_ref().clone(),
            Color::BLACK,
        ));

        ctx.add(
            simple_rect(
                Wh::new(200.px(), 200.px()),
                Color::BLACK,
                1.0.px(),
                Color::WHITE,
            )
            .attach_event(|event| match event {
                Event::MouseDown { event } => {
                    println!("Mouse Down");
                    set_text.set(format!("Mouse Down: {:?}", event));
                }
                Event::MouseMove { event } => {
                    set_text.set(format!("Mouse Move: {:?}", event));
                }
                Event::MouseUp { event } => {
                    println!("Mouse Up");
                    set_text.set(format!("Mouse Up: {:?}", event));
                }
                _ => {}
            }),
        );
    }
}
