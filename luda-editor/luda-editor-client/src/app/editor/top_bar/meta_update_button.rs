use crate::app::types::meta::MetaContainerEvent;
use namui::{
    render, Color, FontType, FontWeight, Language, RectFill, RectParam, RectRound, RectStyle,
    RenderingTree, TextAlign, TextBaseline, TextStyle, Wh,
};

pub struct MetaUpdateButtonProps {
    pub wh: Wh<f32>,
}

pub fn render_meta_update_button(props: &MetaUpdateButtonProps) -> RenderingTree {
    render![
        namui::rect(RectParam {
            x: 0.0,
            y: 0.0,
            width: props.wh.width,
            height: props.wh.height,
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill {
                    color: Color::from_u8(107, 185, 240, 255),
                }),
                round: Some(RectRound { radius: 4.0 }),
            },
        })
        .with_mouse_cursor(namui::MouseCursor::Pointer)
        .attach_event(move |builder| {
            builder.on_mouse_down(move |_| {
                namui::event::send(MetaContainerEvent::MetaReloadRequested)
            });
        }),
        namui::text(namui::TextParam {
            text: "Update Meta".to_string(),
            x: props.wh.width / 2.0,
            y: props.wh.height / 2.0,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                serif: false,
                size: (props.wh.height / 3.0 * 2.0) as i16,
                language: Language::Ko,
                font_weight: FontWeight::REGULAR,
            },
            style: TextStyle {
                color: Color::from_u8(255, 255, 255, 255),
                ..Default::default()
            },
        })
    ]
}
