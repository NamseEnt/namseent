use crate::{list_view, simple_rect, typography};
use namui::prelude::*;
use std::any::Any;

pub struct Dropdown {
    id: Uuid,
    is_opened: bool,
    list_view: list_view::ListView,
    mouse_over_item_index: Option<usize>,
}

pub enum Event {}

enum InternalEvent {
    ToggleDropdown { id: Uuid },
    MoveOverItem { id: Uuid, item_index: usize },
    CloseDropdown { id: Uuid },
}

#[derive(Debug, Clone)]
pub struct Item<TOnSelectItem>
where
    TOnSelectItem: Into<ClosurePtr<(), ()>>,
{
    pub text: String,
    pub is_selected: bool,
    pub on_select_item: TOnSelectItem,
}

pub struct Props<TItems, TOnSelectItem>
where
    TOnSelectItem: Into<ClosurePtr<(), ()>>,
    TItems: IntoIterator<Item = Item<TOnSelectItem>>,
{
    pub rect: Rect<Px>,
    /// Only first `is_selected = true` item will be displayed.
    pub items: TItems,
    /// if `visible_item_count = 0`, all items will be displayed.
    pub visible_item_count: usize,
}

struct InternalItem {
    text: String,
    is_selected: bool,
    on_select_item: ClosurePtr<(), ()>,
}

struct InternalProps {
    pub rect: Rect<Px>,
    /// Only first `is_selected = true` item will be displayed.
    pub items: Vec<InternalItem>,
    /// if `visible_item_count = 0`, all items will be displayed.
    pub visible_item_count: usize,
}

pub fn render<TItems, TOnSelectItem>(props: Props<TItems, TOnSelectItem>) -> RenderingTree
where
    TItems: IntoIterator<Item = Item<TOnSelectItem>>,
    TOnSelectItem: Into<ClosurePtr<(), ()>>,
{
    let internal_props = InternalProps {
        rect: props.rect,
        items: props
            .items
            .into_iter()
            .map(|item| InternalItem {
                text: item.text,
                is_selected: item.is_selected,
                on_select_item: item.on_select_item.into(),
            })
            .collect(),
        visible_item_count: props.visible_item_count,
    };
    react::<Dropdown, _, _>(
        |_| {
            Box::new(Dropdown {
                id: namui::uuid(),
                is_opened: false,
                list_view: list_view::ListView::new(),
                mouse_over_item_index: None,
            }) as Box<dyn React>
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
                    props.rect.wh().height,
                    selected_text.unwrap_or_default(),
                    Color::BLACK,
                ),
            ),
            typography::body::right(props.rect.wh(), "▼", Color::BLACK),
        ])
        .attach_event(move |builder| {
            builder
                .on_mouse_down_in(move |event: MouseEvent| {
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
                        items: props.items.iter().enumerate(),
                        item_render: move |wh, (item_index, item)| {
                            let is_mouse_over = self.mouse_over_item_index == Some(item_index);
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
                                    wh.height,
                                    &item.text,
                                    if is_mouse_over || is_selected {
                                        Color::WHITE
                                    } else {
                                        Color::BLACK
                                    },
                                ),
                            );
                            let on_select_item = item.on_select_item.clone();
                            namui::render([background, text]).attach_event(move |builder| {
                                builder.on_mouse_move_in(move |_| {
                                    namui::event::send(InternalEvent::MoveOverItem {
                                        id,
                                        item_index,
                                    });
                                });

                                let on_select_item = on_select_item.clone();
                                builder.on_mouse_down_in(move |event: MouseEvent| {
                                    event.stop_propagation();
                                    if !is_selected {
                                        on_select_item.invoke(());
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

    fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| match *event {
            InternalEvent::ToggleDropdown { id } => {
                if id == self.id {
                    self.is_opened = !self.is_opened;
                    self.mouse_over_item_index = None;
                }
            }
            InternalEvent::MoveOverItem { id, item_index } => {
                if id == self.id {
                    self.mouse_over_item_index = Some(item_index);
                }
            }
            InternalEvent::CloseDropdown { id } => {
                if id == self.id {
                    self.is_opened = false;
                    self.mouse_over_item_index = None;
                }
            }
        });

        self.list_view.update(event);
    }
}
