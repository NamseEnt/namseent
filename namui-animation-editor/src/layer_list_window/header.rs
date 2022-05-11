use namui::{prelude::*, types::PixelSize};

pub(crate) struct Header {}

pub(crate) struct Props {
    pub wh: Wh<PixelSize>,
}

impl Header {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub(crate) fn update(&mut self, _event: &dyn std::any::Any) {}
    pub(crate) fn render(&self, props: &Props) -> namui::RenderingTree {
        let button_xywh: XywhRect<PixelSize> = XywhRect {
            x: 2.0.into(),
            y: 2.0.into(),
            width: props.wh.height - 4.0,
            height: props.wh.height - 4.0,
        };
        let add_layer_button = render![
            namui::rect(RectParam {
                x: button_xywh.x.into(),
                y: button_xywh.y.into(),
                width: button_xywh.width.into(),
                height: button_xywh.height.into(),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::WHITE,
                    }),
                    ..Default::default()
                },
            }),
            namui::text(TextParam {
                text: "+".to_string(),
                x: f32::from(button_xywh.x + button_xywh.width / 2.0),
                y: f32::from(button_xywh.y + button_xywh.height / 2.0),
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: (f32::from(button_xywh.height) * 0.8) as i16,
                },
                style: TextStyle {
                    color: Color::BLACK,
                    ..Default::default()
                },
            })
        ]
        .attach_event(|builder| {
            builder.on_mouse_up(|_event| {
                namui::event::send(crate::Event::AddLayerButtonClicked);
            })
        });

        let header = render![
            namui::rect(RectParam {
                x: 0.0,
                y: 0.0,
                width: props.wh.width.into(),
                height: props.wh.height.into(),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::BLACK,
                    }),
                    ..Default::default()
                },
            }),
            namui::text(TextParam {
                text: "Layers".to_string(),
                x: f32::from(props.wh.width / 2.0),
                y: f32::from(props.wh.height / 2.0),
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: (f32::from(button_xywh.height) * 0.8) as i16,
                },
                style: TextStyle {
                    color: Color::WHITE,
                    ..Default::default()
                }
            })
        ];

        render![header, add_layer_button]
    }
}
