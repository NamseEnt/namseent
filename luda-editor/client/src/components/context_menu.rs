use namui::prelude::*;
use namui_prebuilt::*;
use std::sync::Arc;

pub struct Item {
    id: namui::Uuid,
    text: String,
    on_click: Arc<dyn Fn()>,
}

impl Item {
    pub fn new(text: impl AsRef<str>, on_click: impl Fn() + 'static) -> Self {
        Self {
            id: uuid(),
            text: text.as_ref().to_string(),
            on_click: Arc::new(on_click),
        }
    }
}

pub struct ContextMenu {
    global_xy: Xy<Px>,
    mouse_over_item_id: Option<Uuid>,
    items: Vec<Item>,
}

pub enum Event {
    Close,
}

enum InternalEvent {
    MouseOver { item_id: namui::Uuid },
    MouseOverClear,
}

impl ContextMenu {
    pub fn new(global_xy: Xy<Px>, items: impl IntoIterator<Item = Item>) -> Self {
        Self {
            global_xy,
            mouse_over_item_id: None,
            items: items.into_iter().collect(),
        }
    }
    pub fn render(&self) -> RenderingTree {
        let cell_wh = Wh::new(160.px(), 24.px());

        let menus = self.items.iter().enumerate().map(|(index, item)| {
            let y = cell_wh.height * index;
            let is_selected = self.mouse_over_item_id.as_ref() == Some(&item.id);
            let border = if is_selected {
                simple_rect(cell_wh, Color::BLACK, 1.px(), Color::WHITE)
            } else {
                simple_rect(cell_wh, Color::WHITE, 1.px(), Color::BLACK)
            };
            let text_color = if is_selected {
                Color::BLACK
            } else {
                Color::WHITE
            };

            translate(
                0.px(),
                y,
                render([
                    border,
                    typography::body::left(cell_wh.height, format!("  {}", item.text), text_color),
                ]),
            )
            .attach_event(move |builder| {
                let item_id = item.id;
                let on_click = item.on_click.clone();
                builder
                    .on_mouse_move_in(move |_| {
                        namui::event::send(InternalEvent::MouseOver { item_id })
                    })
                    .on_mouse_down_in(move |event| {
                        if let Some(MouseButton::Left) = event.button {
                            event.stop_propagation();
                            (on_click)();
                            namui::event::send(Event::Close);
                        }
                    })
                    .on_mouse_down_out(|_| {
                        namui::event::send(Event::Close);
                    });
            })
        });

        on_top(
            absolute(self.global_xy.x, self.global_xy.y, render(menus)).attach_event(
                move |builder| {
                    let is_mouse_over_something = self.mouse_over_item_id.is_some();
                    builder.on_mouse_move_out(move |_| {
                        if is_mouse_over_something {
                            namui::event::send(InternalEvent::MouseOverClear);
                        }
                    });
                },
            ),
        )
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                &InternalEvent::MouseOver { item_id } => {
                    self.mouse_over_item_id = Some(item_id);
                }
                InternalEvent::MouseOverClear => {
                    self.mouse_over_item_id = None;
                }
            }
        }
    }
}
