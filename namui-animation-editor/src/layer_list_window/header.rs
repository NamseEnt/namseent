use namui::prelude::*;

pub(crate) struct Header {}

impl Header {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
pub struct Props {
    pub wh: Wh<Px>,
}

impl Header {
    pub fn update(&mut self, _event: &dyn std::any::Any) {}
    pub fn render(&self, props: Props) -> RenderingTree {
        let button_rect = Rect::Xywh {
            x: px(2.0),
            y: px(2.0),
            width: props.wh.height - px(4.0),
            height: props.wh.height - px(4.0),
        };
        let add_layer_button = render![
            namui::rect(RectParam {
                rect: button_rect,
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::WHITE,
                    }),
                    ..Default::default()
                },
            }),
            namui::text(TextParam {
                text: "+".to_string(),
                x: button_rect.center().x,
                y: button_rect.center().y,
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: (button_rect.height() * 0.8).into(),
                },
                style: TextStyle {
                    color: Color::BLACK,
                    ..Default::default()
                },
                max_width: None,
            })
        ]
        .attach_event(|builder| {
            builder.on_mouse_up_in(|_event| {
                namui::event::send(super::Event::AddLayerButtonClicked);
            });
        });

        let header = render![
            namui::rect(RectParam {
                rect: Rect::from_xy_wh(Xy::single(px(0.0)), props.wh,),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::BLACK,
                    }),
                    ..Default::default()
                },
            }),
            namui::text(TextParam {
                text: "Layers".to_string(),
                x: props.wh.width / 2.0,
                y: props.wh.height / 2.0,
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: (button_rect.height() * 0.8).into(),
                },
                style: TextStyle {
                    color: Color::WHITE,
                    ..Default::default()
                },
                max_width: None,
            })
        ];

        render![header, add_layer_button]
    }
}
