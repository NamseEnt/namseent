use namui::*;
use namui_prebuilt::simple_rect;

pub fn main() {
    namui::start(render)
}

fn render(ctx: &RenderCtx) {
    let cursors = [
        StandardCursor::Default,
        StandardCursor::Pointer,
        StandardCursor::Wait,
        StandardCursor::Progress,
        StandardCursor::Help,
        StandardCursor::Text,
        StandardCursor::VerticalText,
        StandardCursor::NResize,
        StandardCursor::SResize,
        StandardCursor::EResize,
        StandardCursor::WResize,
        StandardCursor::NeResize,
        StandardCursor::NwResize,
        StandardCursor::SeResize,
        StandardCursor::SwResize,
        StandardCursor::EwResize,
        StandardCursor::NsResize,
        StandardCursor::NeswResize,
        StandardCursor::NwseResize,
        StandardCursor::ColResize,
        StandardCursor::RowResize,
        StandardCursor::Move,
        StandardCursor::AllScroll,
        StandardCursor::Grab,
        StandardCursor::Copy,
        StandardCursor::Alias,
        StandardCursor::NoDrop,
        StandardCursor::NotAllowed,
        StandardCursor::Crosshair,
        StandardCursor::Cell,
        StandardCursor::ContextMenu,
        StandardCursor::ZoomIn,
        StandardCursor::ZoomOut,
        StandardCursor::ColorPicker,
        StandardCursor::Pencil,
        StandardCursor::UpArrow,
        StandardCursor::DownArrow,
        StandardCursor::LeftArrow,
        StandardCursor::RightArrow,
    ];

    ctx.translate(namui::mouse::position()).add(simple_rect(
        Wh::one(),
        Color::TRANSPARENT,
        0.px(),
        Color::RED,
    ));
    const HEIGHT: usize = 6;
    for x in 0..7 {
        for y in 0..HEIGHT {
            let Some(cursor) = cursors.get(x * HEIGHT + y) else {
                continue;
            };

            ctx.compose(move |ctx| {
                ctx.translate(((x as i32 * 100).px(), (y as i32 * 100).px()))
                    .mouse_cursor(MouseCursor::Standard(*cursor))
                    .add(simple_rect(
                        Wh::new(100.px(), 100.px()),
                        Color::BLACK,
                        1.px(),
                        Color::WHITE,
                    ));
            });
        }
    }
}
