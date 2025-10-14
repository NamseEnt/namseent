mod draw;
mod images;

use draw::*;
use namui_rendering_tree::*;
use namui_skia::*;
use namui_type::*;
use std::{cell::RefCell, sync::OnceLock};

fn main() {}

thread_local! {
    static SKIA: RefCell<Option<NativeSkia>> = const { RefCell::new(None) };
}
static STANDARD_CURSOR_SPRITE_SET: OnceLock<StandardCursorSpriteSet> = OnceLock::new();

#[unsafe(no_mangle)]
pub extern "C" fn _init_skia(screen_id: usize, window_width: usize, window_height: usize) {
    let skia = init_skia(
        screen_id,
        Wh::new(int_px(window_width as i32), int_px(window_height as i32)),
    )
    .unwrap();

    SKIA.with(|skia_cell| {
        *skia_cell.borrow_mut() = Some(skia);
    });
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _init_standard_cursor_sprite_set(
    metadata_bytes_ptr: *const u8,
    metadata_bytes_len: usize,
) {
    let metadata_slice =
        unsafe { std::slice::from_raw_parts(metadata_bytes_ptr, metadata_bytes_len) };
    let metadata_text = str::from_utf8(metadata_slice).unwrap();

    let standard_cursor_sprite_set =
        StandardCursorSpriteSet::parse(Image::STANDARD_CURSOR_SPRITE_SET, metadata_text).unwrap();

    STANDARD_CURSOR_SPRITE_SET
        .set(standard_cursor_sprite_set)
        .unwrap_or_else(|_| unreachable!("STANDARD_CURSOR_SPRITE_SET is already initialized"));
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _draw_rendering_tree(
    rendering_tree_bytes_ptr: *const u8,
    rendering_tree_bytes_len: usize,
    mouse_x: usize,
    mouse_y: usize,
) {
    let slice =
        unsafe { std::slice::from_raw_parts(rendering_tree_bytes_ptr, rendering_tree_bytes_len) };
    let (rendering_tree, _): (RenderingTree, usize) =
        bincode::decode_from_slice(slice, bincode::config::standard()).unwrap();

    SKIA.with_borrow_mut(|skia_cell| {
        let skia = skia_cell.as_mut().unwrap();
        skia.surface().canvas().clear(Color::WHITE);

        let mouse_xy = Xy::new(px(mouse_x as f32), px(mouse_y as f32));

        let mouse_cursor = calculate_mouse_cursor(&rendering_tree, mouse_xy);

        rendering_tree.draw(skia);

        draw_mouse_cursor(
            skia,
            mouse_xy,
            mouse_cursor,
            STANDARD_CURSOR_SPRITE_SET.get().unwrap(),
        );

        skia.surface().flush();
    });
}

fn calculate_mouse_cursor(rendering_tree: &RenderingTree, mouse_xy: Xy<Px>) -> MouseCursor {
    let mut mouse_cursor = MouseCursor::Standard(StandardCursor::Default);

    let _ = rendering_tree.visit_rln(
        &mut |rendering_tree, tool| {
            let RenderingTree::Special(SpecialRenderingNode::MouseCursor(MouseCursorNode {
                cursor,
                rendering_tree,
            })) = rendering_tree
            else {
                return std::ops::ControlFlow::Continue(());
            };
            let local_xy = tool.to_local_xy(mouse_xy);
            if rendering_tree.xy_in(local_xy) {
                mouse_cursor = *cursor.clone();
            }
            std::ops::ControlFlow::Continue(())
        },
        &[],
    );

    mouse_cursor
}
