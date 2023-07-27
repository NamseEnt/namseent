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

static COUNT_ATOM: Atom<usize> = Atom::uninitialized_new();

impl Component for MyComponent {
    fn render(&self) -> RenderDone {
        let (count, set_count) = use_atom_init(&COUNT_ATOM, || 0);
        // let (count, set_count) = use_state(|| 0);
        let count_mul_2 = use_memo(|| *count * 2);

        let fibo = use_memo(|| get_fibo(*count));
        let fibo2 = use_memo(|| get_fibo(*count_mul_2));

        let text = use_memo(|| format!("Count: {}, Fibo: {}, Fibo2: {}", *count, *fibo, *fibo2));

        use_render_with_event(
            |event| match event {
                Event::OnClick => set_count.mutate(|count| *count += 1),
            },
            |ctx| {
                ctx.add(Button {
                    text,
                    on_click: ctx.event(Event::OnClick),
                });
                if *count % 2 == 0 {
                    ctx.add(StringText { text });
                } else {
                    ctx.add(UsizeText {
                        usize: count.clone(),
                    });
                }
            },
        )
    }
}

impl StaticType for MyComponent {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<MyComponent>()
    }
}

fn get_fibo(x: usize) -> usize {
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
    text: Sig<'a, String>,
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

        use_effect("On button render", || {});

        use_render(|ctx| {
            ctx.add(button::text_button(
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
            ));
        })
    }
}

#[derive(Debug)]
struct StringText<'a> {
    text: Sig<'a, String>,
}

impl StaticType for StringText<'_> {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<StringText>()
    }
}

impl Component for StringText<'_> {
    fn render(&self) -> RenderDone {
        let (value, set_value) = use_state(|| "hello".to_string());

        use_effect("Print text", || {
            namui::log!("StringText: {}", *value);
        });

        use_render(|ctx| {
            ctx.add(button::text_button(
                Rect::Xywh {
                    x: 10.px(),
                    y: 110.px(),
                    width: 100.px(),
                    height: 50.px(),
                },
                &value,
                Color::BLACK,
                Color::BLACK,
                1.px(),
                Color::RED,
                [MouseButton::Left],
                closure(move |_| {}),
            ));
        })
    }
}

#[derive(Debug)]
struct UsizeText<'a> {
    usize: Sig<'a, usize>,
}

impl StaticType for UsizeText<'_> {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<UsizeText>()
    }
}

impl Component for UsizeText<'_> {
    fn render(&self) -> RenderDone {
        let (value, set_value) = use_state(|| 0);
        use_effect("Print text", || {
            namui::log!("UsizeText: {}", *value);
        });

        use_render(|ctx| {
            ctx.add(button::text_button(
                Rect::Xywh {
                    x: 10.px(),
                    y: 210.px(),
                    width: 100.px(),
                    height: 50.px(),
                },
                &value.to_string(),
                Color::BLACK,
                Color::BLACK,
                1.px(),
                Color::RED,
                [MouseButton::Left],
                closure(move |_| {}),
            ));
        })
    }
}
