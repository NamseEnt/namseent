use namui::prelude::*;
use std::sync::Arc;

pub struct ButtonProps<'a> {
    pub xywh: &'a XywhRect<f32>,
    pub text: &'a str,
    pub selected: bool,
}

pub fn render_button(
    ButtonProps {
        xywh,
        text,
        selected,
    }: &ButtonProps,
    on_click: Arc<impl Fn() + 'static>,
) -> Rendering {
    let rect = namui::rect(RectParam {
        x: 0.0,
        y: 0.0,
        width: xywh.width,
        height: xywh.height,
        style: RectStyle {
            stroke: Some(RectStroke {
                color: Color::BLACK,
                width: 1.0,
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
        xywh.x,
        xywh.y,
        render![
            rect,
            namui::text(TextParam {
                x: xywh.width / 2.0,
                y: xywh.height / 2.0,
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
                    size: (xywh.height * 0.8) as i16,
                }
            }),
        ],
    )
}
