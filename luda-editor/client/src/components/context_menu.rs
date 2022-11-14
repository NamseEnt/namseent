use namui::prelude::*;
use namui_prebuilt::*;
use std::sync::Arc;

#[derive(Clone)]
pub enum Item {
    Button {
        id: namui::Uuid,
        text: String,
        on_click: Arc<dyn Fn()>,
    },
    Divider,
}

impl Item {
    pub fn new_button(text: impl AsRef<str>, on_click: impl Fn() + 'static) -> Self {
        Self::Button {
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

        let divider_height = 16.px();
        let mut next_y = 0.px();

        let divider_path = PathBuilder::new()
            .move_to(0.px(), divider_height / 2)
            .line_to(cell_wh.width, divider_height / 2);
        let divider_paint = PaintBuilder::new()
            .set_color(Color::grayscale_f01(0.5))
            .set_stroke_width(1.px())
            .set_style(PaintStyle::Stroke);

        let mut menus = vec![];

        for item in &self.items {
            let y = next_y;
            let menu = match item {
                &Item::Button {
                    id,
                    ref text,
                    ref on_click,
                } => {
                    next_y += cell_wh.height;
                    let is_selected = self.mouse_over_item_id.as_ref() == Some(&id);
                    let border = if is_selected {
                        simple_rect(cell_wh, Color::TRANSPARENT, 0.px(), Color::WHITE)
                    } else {
                        simple_rect(cell_wh, Color::TRANSPARENT, 0.px(), Color::BLACK)
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
                            typography::body::left(
                                cell_wh.height,
                                format!("  {}", text),
                                text_color,
                            ),
                        ]),
                    )
                    .attach_event(move |builder| {
                        let on_click = on_click.clone();
                        builder
                            .on_mouse_move_in(move |_| {
                                namui::event::send(InternalEvent::MouseOver { item_id: id })
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
                }
                Item::Divider => {
                    next_y += divider_height;
                    translate(0.px(), y, path(divider_path.clone(), divider_paint.clone()))
                }
            };
            menus.push(menu);
        }

        let background = simple_rect(
            Wh::new(cell_wh.width, next_y),
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK,
        );

        on_top(absolute(
            self.global_xy.x,
            self.global_xy.y,
            render([background, render(menus)]).attach_event(move |builder| {
                let is_mouse_over_something = self.mouse_over_item_id.is_some();
                builder.on_mouse_move_out(move |_| {
                    if is_mouse_over_something {
                        namui::event::send(InternalEvent::MouseOverClear);
                    }
                });
            }),
        ))
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
