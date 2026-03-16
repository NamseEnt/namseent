use namui::*;
use namui_prebuilt::{button, typography};

pub fn main() {
    namui::start(render)
}

fn render(ctx: &RenderCtx) {
    let (value, set_value) = ctx.state(|| None::<Vec<u8>>);

    const KEY: &str = "abc";

    ctx.effect("load data", || {
        spawn(async move {
            let data = namui::system::kv_store::get(KEY).await;
            set_value.set(data);
        });
    });

    ctx.add(typography::body::left_top(
        match value.as_ref() {
            Some(value) => format!("{value:?}"),
            None => "None".to_string(),
        },
        Color::BLACK,
    ));

    ctx.add(button::TextButton {
        rect: Rect::from_xy_wh(Xy::new(100.px(), 100.px()), Wh::new(100.px(), 40.px())),
        text: "Add 1",
        text_color: Color::BLACK,
        stroke_color: Color::BLACK,
        stroke_width: 1.px(),
        fill_color: Color::WHITE,
        mouse_buttons: vec![MouseButton::Left],
        on_mouse_up_in: |_| {
            set_value.mutate(|value| {
                match value {
                    Some(value) => {
                        value.push(1);
                    }
                    None => {
                        *value = Some(vec![1]);
                    }
                }
                let data = value.clone();
                spawn(async move {
                    namui::system::kv_store::put(KEY, data.as_deref()).await;
                });
            });
        },
    });

    ctx.add(button::TextButton {
        rect: Rect::from_xy_wh(Xy::new(100.px(), 200.px()), Wh::new(100.px(), 40.px())),
        text: "Delete",
        text_color: Color::BLACK,
        stroke_color: Color::BLACK,
        stroke_width: 1.px(),
        fill_color: Color::WHITE,
        mouse_buttons: vec![MouseButton::Left],
        on_mouse_up_in: |_| {
            spawn(async move {
                namui::system::kv_store::put(KEY, None).await;
                set_value.set(None);
            });
        },
    });
}
