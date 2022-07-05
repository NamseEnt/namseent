use namui::prelude::*;
use std::sync::Arc;

pub struct ButtonProps<'a> {
    pub rect: Rect<Px>,
    pub text: &'a str,
    pub selected: bool,
}

pub fn render_button(
    ButtonProps {
        rect,
        text,
        selected,
    }: &ButtonProps,
    on_click: Arc<impl Fn() + 'static>,
) -> Rendering {
    let border = namui::rect(RectParam {
        rect: Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: rect.width(),
            height: rect.height(),
        },
        style: RectStyle {
            stroke: Some(RectStroke {
                color: Color::BLACK,
                width: px(1.0),
                border_position: BorderPosition::Inside,
            }),
            fill: Some(RectFill {
                color: if *selected {
                    Color::BLACK
                } else {
                    Color::WHITE
                },
            }),
            ..Default::default()
        },
        ..Default::default()
    })
    .attach_event(|builder| {
        let on_click = on_click.clone();
        builder.on_mouse_up(move |event| {
            if event.button == Some(MouseButton::Left) {
                on_click();
            }
        });
    });
    translate(
        rect.x(),
        rect.y(),
        render([
            border,
            namui::text(TextParam {
                x: rect.width() / 2.0,
                y: rect.height() / 2.0,
                text: text.to_string(),
                style: TextStyle {
                    color: if *selected {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    },
                    ..Default::default()
                },
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: (rect.height() * 0.8).into(),
                },
            }),
        ]),
    )
}
