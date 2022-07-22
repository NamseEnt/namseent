use super::*;

pub struct BrowserItemProps {
    pub name: String,
    pub thumbnail: Option<Arc<Image>>,
    pub item: ImageBrowserItem,
    pub is_selected: bool,
    pub item_size: Wh<Px>,
    pub thumbnail_rect: Rect<Px>,
    pub browser_id: String,
}

pub fn render_browser_item(props: &BrowserItemProps) -> RenderingTree {
    render([
        rect(RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: props.item_size.width,
                height: props.item_size.height,
            },
            style: RectStyle {
                stroke: Some(RectStroke {
                    width: if props.is_selected { px(3.0) } else { px(1.0) },
                    border_position: BorderPosition::Inside,
                    color: if props.is_selected {
                        namui::Color::RED
                    } else {
                        namui::Color::BLACK
                    },
                }),
                round: Some(RectRound { radius: px(5.0) }),
                fill: Some(RectFill {
                    color: namui::Color::WHITE,
                }),
                ..Default::default()
            },
            ..Default::default()
        })
        .attach_event(move |builder| {
            let item = props.item.clone();
            let browser_id = props.browser_id.clone();
            builder.on_mouse_down_in(move |_| {
                namui::event::send(ImageBrowserEvent::Select {
                    browser_id: browser_id.clone(),
                    item: item.clone(),
                });
            });
        }),
        text(TextParam {
            x: props.item_size.width / 2.0,
            y: props.item_size.height - px(20.0),
            text: props.name.clone(),
            align: TextAlign::Center,
            baseline: TextBaseline::Top,
            font_type: FontType {
                font_weight: FontWeight::REGULAR,
                language: Language::Ko,
                serif: false,
                size: int_px(16),
            },
            style: TextStyle {
                color: namui::Color::BLACK,
                ..Default::default()
            },
        }),
        props
            .thumbnail
            .as_ref()
            .map_or(RenderingTree::Empty, |thumbnail| {
                image(ImageParam {
                    rect: props.thumbnail_rect,
                    source: namui::ImageSource::Image(thumbnail.clone()),
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint_builder: None,
                    },
                })
            }),
    ])
}
