mod draw;

use draw::*;
use namui_skia::*;
use namui_type::*;

pub fn draw(skia: &mut dyn SkSkia, bytes: &[u8], start_load_image: &dyn Fn(&ImageSource)) {
    let input = DrawInput::from_postcard_bytes(bytes);
    let rendering_tree = input.rendering_tree;

    let mut ctx = { DrawContext::new(skia, start_load_image) };

    ctx.canvas().clear(Color::WHITE);
    rendering_tree.draw(&mut ctx);
    ctx.surface().flush();
}
