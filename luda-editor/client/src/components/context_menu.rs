use namui::prelude::*;
use namui_prebuilt::*;

pub fn use_context_menu<'a>(global_xy: Xy<Px>, close: impl 'a + Fn()) -> ContextMenuBuilder<'a> {
    ContextMenuBuilder {
        global_xy,
        items: Default::default(),
        close: Box::new(close),
    }
}

pub struct ContextMenuBuilder<'a> {
    global_xy: Xy<Px>,
    items: Vec<Item<'a>>,
    close: Box<dyn 'a + Fn()>,
}

impl<'a> ContextMenuBuilder<'a> {
    pub fn add_button(
        mut self,
        text: impl AsRef<str>,
        on_click: impl 'a + Fn() + Send + Sync,
    ) -> Self {
        self.items.push(Item::Button {
            text: text.as_ref().to_string(),
            on_click: Box::new(on_click),
        });
        self
    }
    pub fn build(self) -> ContextMenu<'a> {
        ContextMenu {
            global_xy: self.global_xy,
            items: self.items,
            close: self.close,
        }
    }
}

enum Item<'a> {
    Button {
        text: String,
        on_click: Box<dyn 'a + Fn() + Send + Sync>,
    },

    #[allow(dead_code)]
    Divider,
}

impl std::fmt::Debug for Item<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Button { text, .. } => f.debug_struct("Button").field("text", text).finish(),
            Item::Divider => f.debug_struct("Divider").finish(),
        }
    }
}

#[namui::component]
pub struct ContextMenu<'a> {
    global_xy: Xy<Px>,
    items: Vec<Item<'a>>,
    close: Box<dyn 'a + Fn()>,
}

impl Component for ContextMenu<'_> {
    fn render<'a>(&'a self, ctx: RenderCtx<'a>) -> RenderDone {
        let &Self {
            ref items,
            ref close,
            ..
        } = self;
        let (mouse_over_item_idx, set_mouse_over_item_idx) = ctx.use_state(|| None);
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

        let mut menus = vec![];

        for (index, item) in items.iter().enumerate() {
            let y = next_y;
            let menu = match item {
                &Item::Button {
                    ref text,
                    ref on_click,
                } => {
                    next_y += cell_wh.height;
                    let is_mouse_over = *mouse_over_item_idx == Some(index);
                    let background_with_event_handler = {
                        let fill_color = if is_mouse_over {
                            Color::from_u8(129, 198, 232, 255)
                        } else {
                            Color::TRANSPARENT
                        };

                        let close = close.clone();
                        simple_rect(cell_wh, Color::TRANSPARENT, 0.px(), fill_color).attach_event(
                            move |builder| {
                                if is_mouse_over {
                                    builder.on_mouse_move_out(move |_| {
                                        if *mouse_over_item_idx == Some(index)
                                            && *a == Some("Hello".to_string())
                                        {
                                            set_mouse_over_item_idx.set(None);
                                        }
                                    });
                                } else {
                                    builder.on_mouse_move_in(move |_| {
                                        set_mouse_over_item_idx.set(Some(index));
                                    });
                                }
                                builder
                                    .on_mouse_down_in({
                                        let on_click = on_click.clone();
                                        let close = close.clone();
                                        move |event: MouseEvent| {
                                            if let Some(MouseButton::Left) = event.button {
                                                event.stop_propagation();
                                                on_click;
                                                close;
                                            }
                                        }
                                    })
                                    .on_mouse_down_out(move |_| {
                                        close;
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

        ctx.add(on_top(absolute(
            global_xy_within_screen.x,
            global_xy_within_screen.y,
            render([background, render(menus)]).attach_event(move |builder| {
                builder.on_mouse_move_out(move |_| set_mouse_over_item_idx.set(None));
            }),
        )));

        ctx.done()
    }
}

impl ContextMenu<'_> {
    fn global_xy_within_screen(&self, context_menu_wh: Wh<Px>) -> Xy<Px> {
        let screen_wh = namui::screen::size();
        Xy {
            x: (screen_wh.width - context_menu_wh.width).min(self.global_xy.x),
            y: (screen_wh.height - context_menu_wh.height).min(self.global_xy.y),
        }
    }
}
