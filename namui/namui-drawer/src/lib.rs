mod draw;

use draw::*;
use namui_skia::*;
use namui_type::*;

pub fn draw(skia: &mut impl SkSkia, rendering_tree: RenderingTree, mouse_xy: Xy<Px>) {
    skia.move_to_next_frame();

    skia.surface().canvas().clear(Color::WHITE);

    let mouse_cursor = rendering_tree.calculate_mouse_cursor(skia, mouse_xy);

    rendering_tree.draw(skia);

    // Draw mouse cursor
    {
        skia.surface().canvas().save();
        skia.surface().canvas().translate(mouse_xy.x, mouse_xy.y);
        mouse_cursor.draw(skia);
        skia.surface().canvas().restore();
    }

    skia.surface().flush();
}
