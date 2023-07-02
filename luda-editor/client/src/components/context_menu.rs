use namui::prelude::*;
use namui_prebuilt::*;

#[derive(Clone)]
pub enum Item {
    Button {
        id: namui::Uuid,
        text: String,
        on_click: ClosurePtr<(), ()>,
    },

    #[allow(dead_code)]
    Divider,
}

impl Item {
    pub fn new_button(text: impl AsRef<str>, on_click: impl Into<ClosurePtr<(), ()>>) -> Self {
        Self::Button {
            id: uuid(),
            text: text.as_ref().to_string(),
            on_click: on_click.into(),
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
    MouseOverOut { item_id: namui::Uuid },
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
                    let is_mouse_over = self.mouse_over_item_id.as_ref() == Some(&id);
                    let background_with_event_handler = {
                        let fill_color = if is_mouse_over {
                            Color::from_u8(129, 198, 232, 255)
                        } else {
                            Color::TRANSPARENT
                        };
                        simple_rect(cell_wh, Color::TRANSPARENT, 0.px(), fill_color).attach_event(
                            move |builder| {
                                let on_click = on_click.clone();
                                if is_mouse_over {
                                    builder.on_mouse_move_out(move |_| {
                                        namui::event::send(InternalEvent::MouseOverOut {
                                            item_id: id,
                                        })
                                    });
                                } else {
                                    builder.on_mouse_move_in(move |_| {
                                        namui::event::send(InternalEvent::MouseOver { item_id: id })
                                    });
                                }
                                builder
                                    .on_mouse_down_in(move |event: MouseEvent| {
                                        if let Some(MouseButton::Left) = event.button {
                                            event.stop_propagation();
                                            on_click.invoke(());
                                            namui::event::send(Event::Close);
                                        }
                                    })
                                    .on_mouse_down_out(|_| {
                                        namui::event::send(Event::Close);
                                    });
                            },
                        )
                    };
                    let text_color = if is_mouse_over {
                        Color::BLACK
                    } else {
                        Color::WHITE
                    };

                    translate(
                        0.px(),
                        y,
                        render([
                            background_with_event_handler,
                            typography::body::left(
                                cell_wh.height,
                                format!("  {}", text),
                                text_color,
                            ),
                        ]),
                    )
                }
                Item::Divider => {
                    next_y += divider_height;
                    translate(0.px(), y, path(divider_path.clone(), divider_paint.clone()))
                }
            };
            menus.push(menu);
        }

        let context_menu_wh = Wh::new(cell_wh.width, next_y);

        let background = simple_rect(
            context_menu_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::grayscale_f01(0.2),
        );

        let global_xy_within_screen = self.global_xy_within_screen(context_menu_wh);
        on_top(absolute(
            global_xy_within_screen.x,
            global_xy_within_screen.y,
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
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| match event {
            &InternalEvent::MouseOver { item_id } => {
                self.mouse_over_item_id = Some(item_id);
            }
            InternalEvent::MouseOverClear => {
                self.mouse_over_item_id = None;
            }
            &InternalEvent::MouseOverOut { item_id } => {
                if self.mouse_over_item_id.as_ref() == Some(&item_id) {
                    self.mouse_over_item_id = None;
                }
            }
        });
    }

    fn global_xy_within_screen(&self, context_menu_wh: Wh<Px>) -> Xy<Px> {
        let screen_wh = namui::screen::size();
        Xy {
            x: (screen_wh.width - context_menu_wh.width).min(self.global_xy.x),
            y: (screen_wh.height - context_menu_wh.height).min(self.global_xy.y),
        }
    }
}
