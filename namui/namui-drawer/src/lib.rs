mod draw;

use draw::*;
use namui_skia::*;
use namui_type::*;

pub fn draw(
    skia: &mut impl SkSkia,
    rendering_tree: RenderingTree,
    mouse_xy: Xy<Px>,
    standard_cursor_sprite_set: &StandardCursorSpriteSet,
) {
    skia.move_to_next_frame();

    skia.surface().canvas().clear(Color::WHITE);

    let mouse_cursor = calculate_mouse_cursor(&rendering_tree, skia, mouse_xy);

    rendering_tree.draw(skia);

    draw_mouse_cursor(skia, mouse_xy, mouse_cursor, standard_cursor_sprite_set);

    skia.surface().flush();
}

fn calculate_mouse_cursor(
    rendering_tree: &RenderingTree,
    calculator: &dyn SkCalculate,
    mouse_xy: Xy<Px>,
) -> MouseCursor {
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
            if rendering_tree.xy_in(calculator, local_xy) {
                mouse_cursor = *cursor.clone();
            }
            std::ops::ControlFlow::Continue(())
        },
        &[],
    );

    mouse_cursor
}
