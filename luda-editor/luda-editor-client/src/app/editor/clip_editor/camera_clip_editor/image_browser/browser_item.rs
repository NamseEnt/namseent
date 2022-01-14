use crate::app::editor::events::EditorEvent;
use namui::prelude::*;

use super::ImageBrowserItem;

pub struct BrowserItem {}

pub struct BrowserItemProps {
    pub name: String,
    pub thumbnail_url: String,
    pub item: ImageBrowserItem,
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
        render![
            rect(RectParam {
                x: 0.0,
                y: 0.0,
                width: props.item_size.width,
                height: props.item_size.height,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        width: if props.is_selected { 3.0 } else { 1.0 },
                        border_position: BorderPosition::Inside,
                        color: if props.is_selected {
                            namui::Color::RED
                        } else {
                            namui::Color::BLACK
                        },
                    }),
                    round: Some(RectRound { radius: 5.0 }),
                    fill: Some(RectFill {
                        color: namui::Color::WHITE,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            })
            .attach_event(move |builder| {
                let item = props.item.clone();
                builder.on_mouse_down(Box::new(move |_| {
                    namui::log(format!("select browser item {:?}", item));
                    namui::event::send(EditorEvent::ImageBrowserSelectEvent {
                        selected_item: item.clone(),
                    });
                }))
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
                source: namui::ImageSource::Url(props.thumbnail_url.clone()),
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint_builder: None,
                },
            }),
        ]
    }
}
