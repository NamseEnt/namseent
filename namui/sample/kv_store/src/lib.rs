use namui::*;
use namui_prebuilt::{button, typography};

pub fn main() {
    namui::start(render)
}

fn render(ctx: &RenderCtx) {
    let (value, set_value) = ctx.state(|| None);

    const KEY: &str = "abc";

    ctx.effect("load data", || {
        set_value.set(namui::system::file::kv_store::get(KEY).unwrap());
    });

    ctx.add(typography::body::left_top(
        match value.as_ref() {
            Some(value) => format!("{:?}", value),
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
        on_mouse_up_in: &|_| {
            set_value.mutate(|value| {
                match value {
                    Some(value) => {
                        value.push(1);
                    }
                    None => {
                        *value = Some(vec![1]);
                    }
                }
                namui::system::file::kv_store::set(KEY, value.as_ref().unwrap()).unwrap();
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
        on_mouse_up_in: &|_| {
            namui::system::file::kv_store::delete(KEY).unwrap();
            set_value.set(namui::system::file::kv_store::get(KEY).unwrap());
        },
    });
}
