use crate::editor::{events::EditorEvent, types::*};
use namui::prelude::*;

pub struct BrowserItem {}

pub struct BrowserItemProps {
    pub name: String,
    pub thumbnail_url: String,
    pub key: String,
    pub is_selected: bool,
    pub item_size: Wh<f32>,
    pub thumbnail_rect: XywhRect<f32>,
}

impl BrowserItem {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {}
    pub fn render(&self, props: &BrowserItemProps) -> RenderingTree {
        let key = props.key.clone();
        render![
            rect(RectParam {
                x: 0.0,
                y: 0.0,
                width: props.item_size.width,
                height: props.item_size.height,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        width: if props.is_selected {
                            3.0
                        } else {
                            1.0
                        },
                        border_position: BorderPosition::Inside,
                        color: if props.is_selected {
                            namui::Color::RED
                        } else {
                            namui::Color::BLACK
                        },
                    }),
                    round: Some(RectRound {
                        radius: 5.0,
                    }),
                    fill: Some(RectFill {
                        color: namui::Color::WHITE,
                    }),
                    ..Default::default()
                },
                on_mouse_down: Some(Box::new(move |_| {
                    namui::log(format!("select browser item {}", key));
                    namui::event::send(Box::new(EditorEvent::ImageBrowserSelectEvent {
                        selected_key: key.clone(),
                    }));
                })),
                ..Default::default()
            }),
            text(TextParam {
                x: props.item_size.width / 2.0,
                y: props.item_size.height - 20.0,
                text: props.name.clone(),
                align: TextAlign::Center,
                baseline: TextBaseline::Top,
                font_type: FontType {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: 16,
                },
                style: TextStyle {
                    color: namui::Color::BLACK,
                    ..Default::default()
                },
            }),
            image(ImageParam {
                xywh: props.thumbnail_rect,
                url: props.thumbnail_url.clone(),
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            }),
        ]
    }
}
