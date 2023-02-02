use crate::app::game::menu;
use namui::{px, render, Color, MouseButton, Px, PxExt, RenderingTree, Wh};
use namui_prebuilt::{
    button::text_button_fit,
    simple_rect,
    table::{self, TableCell},
    typography,
};

const BUTTON_TEXT_COLOR: Color = Color::WHITE;
const BUTTON_FILL_COLOR: Color = Color::GREEN;
const TITLE_SPACING: Px = px(32.0);
const BUTTON_HEIGHT: Px = px(48.0);
const TITLE_HEIGHT: Px = px(128.0);
const BUTTON_PADDING: Px = px(16.0);

pub fn render_start_menu(wh: Wh<Px>) -> RenderingTree {
    render_with_layout(
        wh,
        [
            margin(),
            title(),
            title_spacing(),
            render_start_new_button(BUTTON_HEIGHT),
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
            margin(),
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
        table::FitAlign::CenterMiddle,
        text_button_fit(
            height,
            "Start New",
            BUTTON_TEXT_COLOR,
            Color::TRANSPARENT,
            0.px(),
            BUTTON_FILL_COLOR,
            BUTTON_PADDING,
            [MouseButton::Left],
            |_| namui::event::send(menu::Event::StartNewButtonClicked),
        )
        .with_mouse_cursor(namui::MouseCursor::Pointer),
    )
}

fn margin<'a>() -> TableCell<'a> {
    table::ratio(1, |_| RenderingTree::Empty)
}

fn title<'a>() -> TableCell<'a> {
    table::fixed(TITLE_HEIGHT, |wh| {
        typography::center_text_full_height(wh, "Luda", BUTTON_TEXT_COLOR)
    })
}

fn title_spacing<'a>() -> TableCell<'a> {
    table::fixed(TITLE_SPACING, |_| RenderingTree::Empty)
}
