use namui::prelude::*;

pub(crate) struct Header {}

impl Header {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
pub struct Props {
    pub wh: Wh<f32>,
}

impl Header {
    pub fn update(&mut self, event: &dyn std::any::Any) {}
    pub fn render(&self, props: Props) -> RenderingTree {
        let button_xywh = XywhRect {
            x: 2.0,
            y: 2.0,
            width: props.wh.height - 4.0,
            height: props.wh.height - 4.0,
        };
        let add_layer_button = render![
            namui::rect(RectParam {
                x: button_xywh.x,
                y: button_xywh.y,
                width: button_xywh.width,
                height: button_xywh.height,
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
                namui::event::send(super::Event::AddLayerButtonClicked);
            });
        });

        let header = render![
            namui::rect(RectParam {
                x: 0.0,
                y: 0.0,
                width: props.wh.width,
                height: props.wh.height,
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
