use crate::app::game::{TileExt, map::MapLoader, save_load::SaveLoad};
use namui::*;
use namui_prebuilt::{
    button::text_button_fit,
    simple_rect,
    table::{self, TableCell},
};

const BUTTON_TEXT_COLOR: Color = Color::WHITE;
const BUTTON_FILL_COLOR: Color = Color::GREEN;
const BUTTON_HEIGHT: Px = px(48.0);
const BUTTON_PADDING: Px = px(16.0);
const BUTTON_SPACING: Px = px(8.0);
const MENU_MARGIN_LEFT: Px = px(64.0);

pub fn render_in_game_menu(wh: Wh<Px>) -> RenderingTree {
    render_with_layout(
        wh,
        [
            margin(),
            render_start_new_button(BUTTON_HEIGHT),
            spacing(),
            render_load_button(BUTTON_HEIGHT),
            margin(),
        ],
    )
}

fn render_with_layout<'a>(
    wh: Wh<Px>,
    items: impl IntoIterator<Item = TableCell<'a>> + 'a,
) -> RenderingTree {
    let button_width = wh.width.min(480.px());
    render([
        render_background(wh),
        table::horizontal([
            table::fixed(MENU_MARGIN_LEFT, |_| RenderingTree::Empty),
            table::fixed(button_width, |wh| table::vertical(items)(wh)),
            margin(),
        ])(wh),
    ])
}

fn render_background(wh: Wh<Px>) -> RenderingTree {
    simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::from_u8(0, 0, 0, 128))
}

fn render_start_new_button<'a>(height: Px) -> table::TableCell<'a> {
    table::fit(
        table::FitAlign::LeftTop,
        text_button_fit(
            height,
            "Start New",
            BUTTON_TEXT_COLOR,
            Color::TRANSPARENT,
            0.px(),
            BUTTON_FILL_COLOR,
            BUTTON_PADDING,
            [MouseButton::Left],
            |_| MapLoader::request_laod("first".to_string(), Xy::new(8.tile(), 6.tile())),
        )
        .with_mouse_cursor(namui::MouseCursor::Pointer),
    )
}

fn render_load_button<'a>(height: Px) -> table::TableCell<'a> {
    table::fit(
        table::FitAlign::LeftTop,
        text_button_fit(
            height,
            "Load",
            BUTTON_TEXT_COLOR,
            Color::TRANSPARENT,
            0.px(),
            BUTTON_FILL_COLOR,
            BUTTON_PADDING,
            [MouseButton::Left],
            |_| SaveLoad::request_load(),
        )
        .with_mouse_cursor(namui::MouseCursor::Pointer),
    )
}

fn margin<'a>() -> TableCell<'a> {
    table::ratio(1, |_| RenderingTree::Empty)
}

fn spacing<'a>() -> TableCell<'a> {
    table::fixed(BUTTON_SPACING, |_| RenderingTree::Empty)
}
