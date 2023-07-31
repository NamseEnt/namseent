use namui::prelude::*;
use namui_prebuilt::*;
use std::{any::TypeId, fmt::Debug};

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &MyComponent {})
}

#[derive(Debug)]
pub struct MyComponent {}

// static COUNT_ATOM: Atom<usize> = Atom::uninitialized_new();

impl Component for MyComponent {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        // let (count, set_count) = ctx.use_atom_init(&COUNT_ATOM, || 0);
        let (count, set_count) = ctx.use_state(|| 0);
        let count_mul_2 = ctx.use_memo(|| *count * 2);

        let fibo = ctx.use_memo(|| get_fibo(*count));
        let fibo2 = ctx.use_memo(|| get_fibo(*count_mul_2));

        let text =
            ctx.use_memo(|| format!("Count: {}, Fibo: {}, Fibo2: {}", *count, *fibo, *fibo2));

        ctx.use_effect("Print text", || {
            namui::log!("{}", *text);
        });

        #[derive(Debug)]
        enum InternalEvent {
            OnClick,
            KeyUp { code: namui::Code },
        }

        ctx.use_children(|ctx| {
            ctx.add(Button {
                text,
                on_click: &|| {
                    set_count.mutate(|count| *count += 1);
                },
            });
            if *count % 2 == 0 {
                ctx.add(
                    StringText { text }, // .attach_event(|builder| {
                                         // builder.on_key_up(ctx.event_with_param(|event: KeyboardEvent| {
                                         //     Some(InternalEvent::KeyUp { code: event.code })
                                         // }));
                                         // })
                );
            } else {
                ctx.add(
                    // hooks::translate(
                    // 50.px(),
                    // 50.px(),
                    UsizeText {
                        usize: count.clone(),
                    },
                    // )
                );
            }
            ctx.done()
        })
    }
}

impl StaticType for MyComponent {
    fn static_type_id(&self) -> StaticTypeId {
        StaticTypeId::Single(TypeId::of::<MyComponent>())
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

struct Button<'a> {
    text: Sig<'a, String>,
    on_click: &'a dyn Fn(),
}
impl<'a> Debug for Button<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button").field("text", &self.text).finish()
    }
}

impl StaticType for Button<'_> {
    fn static_type_id(&self) -> StaticTypeId {
        StaticTypeId::Single(TypeId::of::<Button>())
    }
}

impl Component for Button<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        ctx.use_effect("Print text", || {
            namui::log!("{}", *self.text);
        });

        ctx.use_effect("On button render", || {});

        ctx.use_children(|ctx| {
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
                |_| {
                    namui::log!("Button clicked");
                    (self.on_click)()
                },
            ));

            ctx.done()
        })
    }
}

#[derive(Debug)]
struct StringText<'a> {
    text: Sig<'a, String>,
}

impl StaticType for StringText<'_> {
    fn static_type_id(&self) -> StaticTypeId {
        StaticTypeId::Single(TypeId::of::<StringText>())
    }
}

impl Component for StringText<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let (value, _set_value) = ctx.use_state(|| "hello".to_string());

        ctx.use_effect("Print text", || {
            namui::log!("StringText: {}", *value);
        });

        ctx.use_children(|ctx| {
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
                |_| {
                    namui::log!("StringText clicked");
                },
            ));

            ctx.done()
        })
    }
}

#[derive(Debug)]
struct UsizeText<'a> {
    usize: Sig<'a, usize>,
}

impl StaticType for UsizeText<'_> {
    fn static_type_id(&self) -> StaticTypeId {
        StaticTypeId::Single(TypeId::of::<UsizeText>())
    }
}

impl Component for UsizeText<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let (value, _set_value) = ctx.use_state(|| 0);
        ctx.use_effect("Print text", || {
            namui::log!("UsizeText: {}", *value);
        });

        ctx.use_children(|ctx| {
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
                |_| {
                    namui::log!("UsizeText clicked");
                },
            ));

            ctx.done()
        })
    }
}
