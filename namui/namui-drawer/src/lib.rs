pub mod draw;

pub use draw::*;
pub use namui_rendering_tree::*;
pub use namui_skia::*;
pub use namui_type::*;

use std::cell::RefCell;

thread_local! {
    static LAST_RENDERING_TREE: RefCell<Option<RenderingTree>> = const { RefCell::new(None) };
}

/// Draw a rendering tree directly (for native targets).
pub fn draw_rendering_tree(
    skia: &mut NativeSkia,
    rendering_tree: RenderingTree,
    mouse_x: usize,
    mouse_y: usize,
    sprite_set: Option<&StandardCursorSpriteSet>,
) -> MouseCursor {
    LAST_RENDERING_TREE.with(|cell| {
        *cell.borrow_mut() = Some(rendering_tree);
    });
    redraw(skia, mouse_x, mouse_y, sprite_set)
}

/// Redraw the last rendering tree (for native targets).
pub fn redraw(
    skia: &mut NativeSkia,
    mouse_x: usize,
    mouse_y: usize,
    sprite_set: Option<&StandardCursorSpriteSet>,
) -> MouseCursor {
    LAST_RENDERING_TREE.with_borrow_mut(|rendering_tree| {
        let Some(rendering_tree) = rendering_tree else {
            return MouseCursor::Standard(StandardCursor::Default);
        };

        let mouse_xy = Xy::new(px(mouse_x as f32), px(mouse_y as f32));

        let mouse_cursor = calculate_mouse_cursor(rendering_tree, mouse_xy);

        rendering_tree.clone().draw(skia);

        if let Some(sprite_set) = sprite_set {
            draw::draw_mouse_cursor(skia, mouse_xy, mouse_cursor.clone(), sprite_set);
        }

        mouse_cursor
    })
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
