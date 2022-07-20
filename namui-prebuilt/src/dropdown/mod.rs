use crate::{list_view, simple_rect, typography};
use namui::prelude::*;
use std::{any::Any, sync::Arc};

pub struct Dropdown {
    id: Id,
    is_opened: bool,
    list_view: list_view::ListView,
    mouse_over_item_id: Option<String>,
}

pub enum Event {}

enum InternalEvent {
    ToggleDropdown { id: Id },
    MoveOverItem { id: Id, item_id: String },
    CloseDropdown { id: Id },
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

struct InternalProps {
    pub rect: Rect<Px>,
    /// Only first `is_selected = true` item will be displayed.
    pub items: Vec<Item>,
    /// if `visible_item_count = 0`, all items will be displayed.
    pub visible_item_count: usize,
    pub on_select_item: Arc<dyn Fn(String)>,
}

pub fn render<TItems, TOnSelectItem>(props: Props<TItems, TOnSelectItem>) -> RenderingTree
where
    TItems: IntoIterator<Item = Item>,
    TOnSelectItem: Fn(String) + 'static,
{
    let internal_props = InternalProps {
        rect: props.rect,
        items: props.items.into_iter().collect(),
        visible_item_count: props.visible_item_count,
        on_select_item: Arc::new(props.on_select_item),
    };
    react::<Dropdown, _, _>(
        || {
            Box::new(Dropdown {
                id: namui::random_id(),
                is_opened: false,
                list_view: list_view::ListView::new(),
                mouse_over_item_id: None,
            })
        },
        internal_props,
    )
}

impl React for Dropdown {
    fn render(&self, props: &dyn Any) -> RenderingTree {
        let props = props.downcast_ref::<InternalProps>().unwrap();

        let id = self.id;
        const LEFT_PADDING: Px = px(10.0);

        let selected_text = props
            .items
            .iter()
            .find(|item| item.is_selected)
            .map(|item| item.text.clone());

        let head = namui::render([
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
        .attach_event(move |builder| {
            builder
                .on_mouse_down_in(move |event| {
                    event.stop_propagation();
                    namui::event::send(InternalEvent::ToggleDropdown { id })
                })
                .on_mouse_down_out(move |_| {
                    namui::event::send(InternalEvent::CloseDropdown { id })
                });
        });

        let body = if self.is_opened {
            let body_height = props.rect.height()
                * (if props.visible_item_count == 0 {
                    props.items.len()
                } else {
                    props.visible_item_count
                });
            on_top(translate(
                0.px(),
                props.rect.height(),
                namui::render([
                    self.list_view.render(list_view::Props {
                        xy: Xy::zero(),
                        height: body_height,
                        scroll_bar_width: 5.px(),
                        item_wh: props.rect.wh(),
                        items: &props.items,
                        item_render: move |wh, item| {
                            let is_mouse_over = self.mouse_over_item_id.as_ref() == Some(&item.id);
                            let is_selected = item.is_selected;
                            let background = simple_rect(
                                props.rect.wh(),
                                Color::WHITE,
                                0.px(),
                                if is_selected {
                                    Color::BLUE
                                } else if is_mouse_over {
                                    Color::from_u8(0x5C, 0x5C, 255, 255)
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
                                    if is_mouse_over || is_selected {
                                        Color::WHITE
                                    } else {
                                        Color::BLACK
                                    },
                                ),
                            );
                            let on_select_item = props.on_select_item.clone();
                            namui::render([background, text]).attach_event(move |builder| {
                                let item_id = item.id.clone();
                                builder.on_mouse_move_in(move |_| {
                                    namui::event::send(InternalEvent::MoveOverItem {
                                        id,
                                        item_id: item_id.clone(),
                                    });
                                });

                                let item_id = item.id.clone();
                                let on_select_item = on_select_item.clone();
                                builder.on_mouse_down_in(move |event| {
                                    event.stop_propagation();
                                    if !is_selected {
                                        (on_select_item)(item_id.clone());
                                    }
                                    namui::event::send(InternalEvent::CloseDropdown { id });
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
            ))
        } else {
            RenderingTree::Empty
        };
        translate(props.rect.x(), props.rect.y(), namui::render([head, body]))
    }

    fn update(&mut self, event: &dyn Any) {
        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                InternalEvent::ToggleDropdown { id } => {
                    if id == self.id {
                        self.is_opened = !self.is_opened;
                        self.mouse_over_item_id = None;
                    }
                }
                InternalEvent::MoveOverItem { id, item_id } => {
                    if id == self.id {
                        self.mouse_over_item_id = Some(item_id.clone());
                    }
                }
                InternalEvent::CloseDropdown { id } => {
                    if id == self.id {
                        self.is_opened = false;
                        self.mouse_over_item_id = None;
                    }
                }
            }
        }

        self.list_view.update(event);
    }
}
