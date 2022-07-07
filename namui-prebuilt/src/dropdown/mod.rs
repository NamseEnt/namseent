use std::sync::Arc;

use crate::{list_view, simple_rect, typography};
use namui::prelude::*;

pub struct Dropdown {
    id: String,
    is_opened: bool,
    list_view: list_view::ListView,
    mouse_over_item_id: Option<String>,
}

pub enum Event {}

enum InternalEvent {
    ToggleDropdown,
    MoveOverItem { item_id: String },
    CloseDropdown,
}

#[derive(Debug, Clone)]
pub struct Item {
    /// Should be unique.
    pub id: String,
    pub text: String,
    pub is_selected: bool,
}

pub struct Props<TItems, TOnSelectItem>
where
    TItems: IntoIterator<Item = Item>,
    TOnSelectItem: Fn(String) + 'static,
{
    pub rect: Rect<Px>,
    /// Only first `is_selected = true` item will be displayed.
    pub items: TItems,
    /// if `visible_item_count = 0`, all items will be displayed.
    pub visible_item_count: usize,
    pub on_select_item: TOnSelectItem,
}

impl Dropdown {
    pub fn new() -> Self {
        Self {
            id: namui::nanoid(),
            is_opened: false,
            list_view: list_view::ListView::new(),
            mouse_over_item_id: None,
        }
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                InternalEvent::ToggleDropdown => {
                    self.is_opened = !self.is_opened;
                    self.mouse_over_item_id = None;
                }
                InternalEvent::MoveOverItem { item_id } => {
                    namui::log!("MoveOverItem {}", item_id);
                    self.mouse_over_item_id = Some(item_id.clone());
                }
                InternalEvent::CloseDropdown => {
                    self.is_opened = false;
                    self.mouse_over_item_id = None;
                }
            }
        }

        self.list_view.update(event);
    }

    pub fn render<TItems, TOnSelectItem>(
        &self,
        props: Props<TItems, TOnSelectItem>,
    ) -> RenderingTree
    where
        TItems: IntoIterator<Item = Item>,
        TOnSelectItem: Fn(String) + 'static,
    {
        const LEFT_PADDING: Px = px(10.0);

        let items: Vec<_> = props.items.into_iter().collect();

        let selected_text = items
            .iter()
            .find(|item| item.is_selected)
            .map(|item| item.text.clone());

        let head = render([
            simple_rect(props.rect.wh(), Color::BLACK, 1.px(), Color::WHITE),
            translate(
                LEFT_PADDING,
                0.px(),
                typography::body::left(
                    props.rect.wh(),
                    selected_text.unwrap_or_default(),
                    Color::BLACK,
                ),
            ),
            typography::body::right(props.rect.wh(), "▼", Color::BLACK),
        ])
        .attach_event(|builder| {
            builder.on_mouse_down_in(|_| namui::event::send(InternalEvent::ToggleDropdown));
        });
        let on_select_item = Arc::new(props.on_select_item);

        let body = if self.is_opened {
            let body_height = props.rect.height()
                * (if props.visible_item_count == 0 {
                    items.len()
                } else {
                    props.visible_item_count
                });
            translate(
                0.px(),
                props.rect.height(),
                render([
                    self.list_view.render(list_view::Props {
                        xy: Xy::zero(),
                        height: body_height,
                        scroll_bar_width: 5.px(),
                        item_wh: props.rect.wh(),
                        items,
                        item_render: |wh, item| {
                            let is_mouse_over = self.mouse_over_item_id.as_ref() == Some(&item.id);
                            let background = simple_rect(
                                props.rect.wh(),
                                Color::WHITE,
                                0.px(),
                                if is_mouse_over {
                                    Color::BLUE
                                } else {
                                    Color::WHITE
                                },
                            );
                            let text = translate(
                                LEFT_PADDING,
                                0.px(),
                                typography::body::left(
                                    wh,
                                    &item.text,
                                    if is_mouse_over {
                                        Color::WHITE
                                    } else {
                                        Color::BLACK
                                    },
                                ),
                            );
                            let on_select_item = on_select_item.clone();
                            render([background, text]).attach_event(move |builder| {
                                let item_id = item.id.clone();
                                builder.on_mouse_move_in(move |_| {
                                    namui::event::send(InternalEvent::MoveOverItem {
                                        item_id: item_id.clone(),
                                    });
                                });
                                let item_id = item.id.clone();
                                let on_select_item = on_select_item.clone();
                                builder.on_mouse_down_in(move |_| {
                                    (on_select_item)(item_id.clone());
                                    namui::event::send(InternalEvent::CloseDropdown);
                                });
                            })
                        },
                    }),
                    simple_rect(
                        Wh {
                            width: props.rect.width(),
                            height: body_height,
                        },
                        Color::BLACK,
                        1.px(),
                        Color::TRANSPARENT,
                    ),
                ]),
            )
        } else {
            RenderingTree::Empty
        };
        translate(props.rect.x(), props.rect.y(), render([head, body]))
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
