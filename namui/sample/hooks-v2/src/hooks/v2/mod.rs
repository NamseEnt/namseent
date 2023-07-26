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
    fn render(&self) -> RenderDone {
        let (count, set_count) = use_state(|| 0);
        let count_mul_2 = use_memo(|| *count * 2);

        let fibo = use_memo(|| get_fibo(*count));
        let fibo2 = use_memo(|| get_fibo(*count_mul_2));

        let text = use_memo(|| format!("Count: {}, Fibo: {}, Fibo2: {}", *count, *fibo, *fibo2));

        use_render_with_event(
            |event| match event {
                Event::OnClick => set_count.mutate(|count| *count += 1),
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
struct Button<'a> {
    // text: Signal<String>,
    text: Signal<'a, String>,
    on_click: EventCallback,
}

impl StaticType for Button<'_> {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<Button>()
    }
}

impl Component for Button<'_> {
    fn render(&self) -> RenderDone {
        use_effect("Print text", || {
            namui::log!("{}", *self.text);
        });

        // use_effect("On button render", || {});

        use_render(|| {
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
