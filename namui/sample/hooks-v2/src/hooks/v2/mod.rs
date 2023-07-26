pub mod hooks;

use hooks::*;
use namui::prelude::*;
use namui_prebuilt::*;
use std::{any::TypeId, fmt::Debug};

#[derive(Debug)]
pub struct MyComponent {}

enum Event {
    OnClick,
}

impl Component for MyComponent {
    fn component<'a>(&'a self, ctx: &'a Context) -> ContextDone {
        namui::log!("MyComponent");
        let (count, set_count) = ctx.state(|| 0);
        let fibo = ctx.memo(|| get_fibo(*count));
        let text = ctx.memo(|| format!("Count: {}, Fibo: {}", *count, *fibo));
        namui::log!("MyComponent text: {:?}", text);

        ctx.render_with_event(
            |event| match event {
                Event::OnClick => {
                    namui::log!("Clicked");
                    set_count.mutate(|count| *count += 1)
                }
            },
            |ctx| Button {
                text,
                on_click: ctx.event(Event::OnClick),
            },
        )
    }
}

impl StaticType for MyComponent {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<MyComponent>()
    }
}

fn get_fibo(x: u32) -> u32 {
    if x == 0 {
        return 0;
    }
    if x == 1 {
        return 1;
    }
    get_fibo(x - 1) + get_fibo(x - 2)
}

#[derive(Debug)]
struct Button {
    text: Signal<String>,
    on_click: EventCallback,
}

impl StaticType for Button {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<Button>()
    }
}

impl Component for Button {
    fn component<'a>(&'a self, ctx: &'a Context) -> ContextDone {
        namui::log!("Button text: {:?}", self.text);

        ctx.effect("Print text on text effect", || {
            if self.text.on_effect() {
                namui::log!("Count changed");
            }
        });

        ctx.effect("On button render", || {
            namui::log!("Button rendered");
        });

        ctx.render(|| {
            namui::log!("Button Render!!");
            button::text_button(
                Rect::Xywh {
                    x: 10.px(),
                    y: 10.px(),
                    width: 100.px(),
                    height: 50.px(),
                },
                &self.text,
                Color::BLACK,
                Color::BLACK,
                1.px(),
                Color::RED,
                [MouseButton::Left],
                closure({
                    let on_click = self.on_click.clone();
                    move |_| on_click.call()
                }),
            )
        })
    }
}
