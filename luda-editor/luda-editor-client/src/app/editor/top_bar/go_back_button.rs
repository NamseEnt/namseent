use crate::app::editor::top_bar::events::TopBarEvent;
use namui::prelude::*;

pub fn render_go_back_button(wh: Wh<Px>) -> RenderingTree {
    render([
        namui::rect(RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: wh.width,
                height: wh.height,
            },
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill {
                    color: Color::from_u8(107, 185, 240, 255),
                }),
                round: Some(RectRound { radius: px(4.0) }),
            },
        })
        .with_mouse_cursor(namui::MouseCursor::Pointer)
        .attach_event(move |builder| {
            builder.on_mouse_down(move |_| namui::event::send(TopBarEvent::GoBackButtonClicked));
        }),
        namui::text(namui::TextParam {
            text: "Go back".to_string(),
            x: wh.width / 2.0,
            y: wh.height / 2.0,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                serif: false,
                size: (wh.height / 3.0 * 2.0).into(),
                language: Language::Ko,
                font_weight: FontWeight::REGULAR,
            },
            style: TextStyle {
                color: Color::from_u8(255, 255, 255, 255),
                ..Default::default()
            },
        }),
    ])
}
