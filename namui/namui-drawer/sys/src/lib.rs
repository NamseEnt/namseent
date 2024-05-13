mod draw;

use draw::*;
use namui_skia::*;
use namui_type::*;

pub fn draw(skia: &mut impl SkSkia, rendering_tree: RenderingTree) {
    skia.move_to_next_frame();

    skia.surface().canvas().clear(Color::WHITE);
    rendering_tree.draw(skia);
    skia.surface().flush();
}
