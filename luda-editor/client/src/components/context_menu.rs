use namui::prelude::*;
use namui_prebuilt::*;

#[derive(Clone)]
pub enum Item {
    Button {
        id: namui::Uuid,
        text: String,
        on_click: Callback,
    },

    #[allow(dead_code)]
    Divider,
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Button { id, text, .. } => f
                .debug_struct("Button")
                .field("id", id)
                .field("text", text)
                .finish(),
            Item::Divider => f.debug_struct("Divider").finish(),
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Item::Button { id, text, .. },
                Item::Button {
                    id: id2,
                    text: text2,
                    ..
                },
            ) => id == id2 && text == text2,
            (Item::Divider, Item::Divider) => true,
            _ => false,
        }
    }
}

impl Item {
    pub fn new_button(text: impl AsRef<str>, on_click: Callback) -> Self {
        let a = || println!("hi");
        let a = { || println!("hi") };
        Self::Button {
            id: uuid(),
            text: text.as_ref().to_string(),
            on_click,
        }
    }
}

pub fn use_context_menu() -> (Option<ContextMenu2>, SetState<Option<(Xy<Px>, Vec<Item>)>>) {
    let (props, set_props) = ctx.use_state::<Option<(Xy<Px>, Vec<Item>)>>(|| None);

    let context_menu = if let Some((global_xy, items)) = &*props {
        Some(ContextMenu2 {
            global_xy: global_xy.clone(),
            items: items.clone(),
            close: set_props.map_set_callback(None),
        })
    } else {
        None
    };

    (context_menu, set_props)
}

#[namui::component]
#[derive(Clone)]
pub struct ContextMenu2 {
    global_xy: Xy<Px>,
    items: Vec<Item>,
    close: Callback,
}

impl ContextMenu2 {
    pub fn new(global_xy: Xy<Px>, items: impl IntoIterator<Item = Item>, close: Callback) -> Self {
        Self {
            global_xy,
            items: items.into_iter().collect(),
            close,
        }
    }
}

impl Component for ContextMenu2 {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            ref items,
            ref close,
            ..
        } = self;
        let (mouse_over_item_id, set_mouse_over_item_id) = ctx.use_state(|| None);
        let (a, set_a) = ctx.use_state::<Option<String>>(|| None);
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

        ctx.use_children(|ctx| {
            let mut menus = vec![];

            for item in items {
                let y = next_y;
                let menu = match item {
                    &Item::Button {
                        id,
                        ref text,
                        ref on_click,
                    } => {
                        next_y += cell_wh.height;
                        let is_mouse_over = *mouse_over_item_id == Some(id);
                        let background_with_event_handler = {
                            let fill_color = if is_mouse_over {
                                Color::from_u8(129, 198, 232, 255)
                            } else {
                                Color::TRANSPARENT
                            };

                            let close = close.clone();
                            simple_rect(cell_wh, Color::TRANSPARENT, 0.px(), fill_color)
                                .attach_event(move |builder| {
                                    if is_mouse_over {
                                        builder.on_mouse_move_out(move |_| {
                                            if *mouse_over_item_id == Some(id)
                                                && *a == Some("Hello".to_string())
                                            {
                                                set_mouse_over_item_id.set(None);
                                            }
                                        });
                                    } else {
                                        builder.on_mouse_move_in(move |_| {
                                            set_mouse_over_item_id.set(Some(id));
                                        });
                                    }
                                    builder
                                        .on_mouse_down_in({
                                            let on_click = on_click.clone();
                                            let close = close.clone();
                                            move |event: MouseEvent| {
                                                if let Some(MouseButton::Left) = event.button {
                                                    event.stop_propagation();
                                                    on_click.call();
                                                    close.call();
                                                }
                                            }
                                        })
                                        .on_mouse_down_out(move |_| {
                                            close.call();
                                        });
                                })
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

            ctx.add(on_top(absolute(
                global_xy_within_screen.x,
                global_xy_within_screen.y,
                render([background, render(menus)]).attach_event(move |builder| {
                    builder.on_mouse_move_out(move |_| set_mouse_over_item_id.set(None));
                }),
            )));
        })
    }
}

impl ContextMenu2 {
    fn global_xy_within_screen(&self, context_menu_wh: Wh<Px>) -> Xy<Px> {
        let screen_wh = namui::screen::size();
        Xy {
            x: (screen_wh.width - context_menu_wh.width).min(self.global_xy.x),
            y: (screen_wh.height - context_menu_wh.height).min(self.global_xy.y),
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
                                            on_click.call();
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
