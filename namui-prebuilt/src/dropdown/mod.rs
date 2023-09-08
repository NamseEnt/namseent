use crate::{list_view::ListView, simple_rect, typography};
use namui::prelude::*;
use std::{fmt::Debug, ops::Deref};

const LEFT_PADDING: Px = px(10.0);

#[component]
pub struct Dropdown<'a> {
    pub rect: Rect<Px>,
    pub items: Vec<Item<'a>>,
    pub visible_item_count: usize,
}

impl Component for Dropdown<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            rect,
            items,
            visible_item_count,
        } = self;

        let (is_opened, set_is_opened) = ctx.state(|| false);
        let (mouse_over_item_index, set_mouse_over_item_index) = ctx.state(|| None);

        let selected_text = items
            .iter()
            .find(|item| item.is_selected)
            .map(|item| item.text.clone());

        ctx.compose(|ctx| {
            let mut ctx = ctx.translate((rect.x(), rect.y()));

            ctx.compose(|ctx| {
                ctx.add(
                    simple_rect(rect.wh(), Color::BLACK, 1.px(), Color::WHITE).attach_event(
                        move |event| {
                            if let namui::Event::MouseDown { event } = event {
                                match event.is_local_xy_in() {
                                    true => {
                                        event.stop_propagation();
                                        set_is_opened.set(true);
                                        set_mouse_over_item_index.set(None);
                                    }
                                    false => {
                                        set_is_opened.set(false);
                                        set_mouse_over_item_index.set(None);
                                    }
                                }
                            }
                        },
                    ),
                );
                ctx.translate((LEFT_PADDING, 0.px()))
                    .add(typography::body::left(
                        rect.wh().height,
                        selected_text.unwrap_or_default(),
                        Color::BLACK,
                    ));
                ctx.add(typography::body::right(rect.wh(), "▼", Color::BLACK));
            });

            ctx.compose(|ctx| {
                if !is_opened.deref() {
                    return;
                }
                let body_height = rect.height()
                    * (if visible_item_count == 0 {
                        items.len()
                    } else {
                        visible_item_count
                    });

                ctx.on_top()
                    .translate((0.px(), rect.height()))
                    .add(ListView {
                        xy: Xy::zero(),
                        height: body_height,
                        scroll_bar_width: 5.px(),
                        item_wh: rect.wh(),
                        items: items
                            .into_iter()
                            .enumerate()
                            .map(|(item_index, item)| {
                                let is_mouse_over =
                                    mouse_over_item_index.deref() == &Some(item_index);
                                let is_selected = item.is_selected;

                                let item_component = InternalItem {
                                    wh: rect.wh(),
                                    text: item.text,
                                    is_selected,
                                    is_mouse_over,
                                }
                                .attach_event(move |event| match event {
                                    Event::MouseDown { event } => {
                                        if event.is_local_xy_in() {
                                            set_mouse_over_item_index.set(Some(item_index));
                                        }
                                    }
                                    Event::MouseMove { event } => {
                                        if event.is_local_xy_in() {
                                            event.stop_propagation();
                                            if !is_selected {
                                                (item.on_select_item)();
                                            }
                                            set_is_opened.set(false);
                                        }
                                    }
                                    _ => {}
                                });
                                (item_index.to_string(), item_component)
                            })
                            .collect(),
                    })
                    .add(simple_rect(
                        Wh {
                            width: rect.width(),
                            height: body_height,
                        },
                        Color::BLACK,
                        1.px(),
                        Color::TRANSPARENT,
                    ));
            });
        });

        ctx.done()
    }
}

pub struct Item<'a> {
    pub text: String,
    pub is_selected: bool,
    pub on_select_item: Box<dyn 'a + Fn()>,
}
impl Debug for Item<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Item")
            .field("text", &self.text)
            .field("is_selected", &self.is_selected)
            .finish()
    }
}

#[component]
struct InternalItem {
    wh: Wh<Px>,
    text: String,
    is_selected: bool,
    is_mouse_over: bool,
}

impl Component for InternalItem {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            text,
            is_selected,
            is_mouse_over,
        } = self;

        ctx.compose(|ctx| {
            ctx.add(simple_rect(
                wh,
                Color::WHITE,
                0.px(),
                if is_selected {
                    Color::BLUE
                } else if is_mouse_over {
                    Color::from_u8(0x5C, 0x5C, 255, 255)
                } else {
                    Color::WHITE
                },
            ))
            .translate((LEFT_PADDING, 0.px()))
            .add(typography::body::left(
                wh.height,
                &text,
                if is_mouse_over || is_selected {
                    Color::WHITE
                } else {
                    Color::BLACK
                },
            ));
        });

        ctx.done()
    }
}
