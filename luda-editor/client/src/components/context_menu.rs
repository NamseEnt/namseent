use namui::prelude::*;
use namui_prebuilt::*;
use std::any::Any;

static CONTEXT_MENU_ATOM: Atom<Option<ContextMenuData>> = Atom::uninitialized_new();

static mut RECIPE: Option<Box<dyn Any + Send + Sync + 'static>> = None;
static mut GLOBAL_XY: Option<Xy<Px>> = None;
static mut CLICKED_BUTTON_INDEX: Option<usize> = None;

pub fn open_context_menu(global_xy: Xy<Px>, recipe: impl Any + Send + Sync + 'static) {
    unsafe {
        RECIPE.replace(Box::new(recipe));
        GLOBAL_XY.replace(global_xy);
    }
}
pub fn if_context_menu_for<T: 'static>(
    on: impl FnOnce(&T, ContextMenuBuilder) -> ContextMenuBuilder,
) {
    unsafe {
        if let Some(recipe) = RECIPE.as_ref() {
            if let Some(recipe) = recipe.as_ref().downcast_ref::<T>() {
                let clicked_button_index = CLICKED_BUTTON_INDEX.take();
                let builder = on(recipe, ContextMenuBuilder::new(clicked_button_index));

                let context_menu = builder.build(GLOBAL_XY.unwrap());

                if CONTEXT_MENU_ATOM
                    .get_or_init(Default::default)
                    .as_ref()
                    .is_some_and(|x| x == &context_menu)
                {
                    if clicked_button_index.is_some() {
                        close_context_menu();
                    }
                } else {
                    CONTEXT_MENU_ATOM.set(Some(context_menu));
                }
            }
        }
    }
}

fn close_context_menu() {
    unsafe {
        RECIPE.take();
        GLOBAL_XY.take();
    }
    CONTEXT_MENU_ATOM.set(None);
}

fn on_click_context_menu_item(index: usize) {
    unsafe {
        CLICKED_BUTTON_INDEX.replace(index);
    }
}

#[derive(Debug, PartialEq)]
struct ContextMenuData {
    global_xy: Xy<Px>,
    items: Vec<Item>,
}
pub struct ContextMenuBuilder {
    items: Vec<Item>,
    clicked_button_index: Option<usize>,
    button_index: usize,
}

impl ContextMenuBuilder {
    fn new(clicked_button_index: Option<usize>) -> Self {
        Self {
            items: vec![],
            clicked_button_index,
            button_index: 0,
        }
    }
    pub fn add_button(mut self, text: impl AsRef<str>, on_click: impl FnOnce()) -> Self {
        if self.clicked_button_index == Some(self.button_index) {
            on_click();
        }

        self.items.push(Item::Button {
            text: text.as_ref().to_string(),
        });

        self.button_index += 1;
        self
    }
    pub fn and<'then, Modifier>(self, then: Modifier) -> Self
    where
        Modifier: 'then + Fn(Self) -> Self,
    {
        then(self)
    }
    fn build(self, global_xy: Xy<Px>) -> ContextMenuData {
        ContextMenuData {
            global_xy,
            items: self.items,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Item {
    Button {
        text: String,
    },

    #[allow(dead_code)]
    Divider,
}

#[namui::component]
pub struct ContextMenu;

impl Component for ContextMenu {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let (mouse_over_item_idx, set_mouse_over_item_idx) = ctx.state(|| None);
        let (atom, _) = ctx.atom_init(&CONTEXT_MENU_ATOM, Default::default);

        let Some(atom) = atom.as_ref() else {
            return ctx.done();
        };

        let cell_wh = Wh::new(160.px(), 24.px());

        let divider_height = 16.px();
        let mut next_y = 0.px();

        let divider_path = Path::new()
            .move_to(0.px(), divider_height / 2)
            .line_to(cell_wh.width, divider_height / 2);
        let divider_paint = Paint::new()
            .set_color(Color::grayscale_f01(0.5))
            .set_stroke_width(1.px())
            .set_style(PaintStyle::Stroke);

        let ys = atom
            .items
            .iter()
            .map(|item| {
                let y = next_y;

                next_y += match item {
                    Item::Button { .. } => cell_wh.height,
                    Item::Divider => divider_height,
                };

                y
            })
            .collect::<Vec<_>>();

        let menus = |ctx: &mut ComposeCtx| {
            for ((index, item), y) in atom.items.iter().enumerate().zip(ys) {
                match item {
                    Item::Button { text } => {
                        let is_mouse_over = *mouse_over_item_idx == Some(index);
                        let background = {
                            let fill_color = if is_mouse_over {
                                Color::from_u8(129, 198, 232, 255)
                            } else {
                                Color::TRANSPARENT
                            };

                            simple_rect(cell_wh, Color::TRANSPARENT, 0.px(), fill_color)
                        };
                        let text_color = if is_mouse_over {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        };
                        ctx.add_with_key(
                            index.to_string(),
                            translate(
                                0.px(),
                                y,
                                render([
                                    typography::body::left(
                                        cell_wh.height,
                                        format!("  {}", text),
                                        text_color,
                                    ),
                                    background,
                                ]),
                            )
                            .attach_event(|event| match event {
                                Event::MouseUp { event } => {
                                    if event.is_local_xy_in() {
                                        if let Some(MouseButton::Left) = event.button {
                                            event.stop_propagation();
                                            on_click_context_menu_item(index);
                                        }
                                    }
                                }
                                Event::MouseMove { event } => {
                                    if is_mouse_over {
                                        if !event.is_local_xy_in() {
                                            if *mouse_over_item_idx == Some(index) {
                                                set_mouse_over_item_idx.set(None);
                                            }
                                        }
                                    } else {
                                        if event.is_local_xy_in() {
                                            set_mouse_over_item_idx.set(Some(index));
                                        }
                                    }
                                }
                                _ => {}
                            })
                            .with_mouse_cursor(MouseCursor::Pointer),
                        );
                    }
                    Item::Divider => {
                        ctx.add_with_key(
                            index.to_string(),
                            translate(0.px(), y, path(divider_path.clone(), divider_paint.clone())),
                        );
                    }
                }
            }
        };

        let context_menu_wh = Wh::new(cell_wh.width, next_y);

        let background = simple_rect(
            context_menu_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::grayscale_f01(0.2),
        )
        .attach_event(|event| match event {
            Event::MouseDown { event } => {
                event.stop_propagation();
                if !event.is_local_xy_in() {
                    close_context_menu();
                }
            }
            _ => {}
        });

        let global_xy_within_screen = global_xy_within_screen(atom.global_xy, context_menu_wh);

        ctx.compose(|ctx| {
            ctx.on_top()
                .absolute(global_xy_within_screen)
                .compose(menus)
                .add(background);
        });

        ctx.done()
    }
}

fn global_xy_within_screen(global_xy: Xy<Px>, context_menu_wh: Wh<Px>) -> Xy<Px> {
    let screen_wh = namui::screen::size();
    Xy {
        x: (screen_wh.width - context_menu_wh.width).min(global_xy.x),
        y: (screen_wh.height - context_menu_wh.height).min(global_xy.y),
    }
}
