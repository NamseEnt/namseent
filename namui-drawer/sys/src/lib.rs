mod draw;

use draw::*;
use namui_skia::*;
use namui_type::*;

pub fn draw(skia: &mut impl SkSkia, input: DrawInput, start_load_image: &dyn Fn(&ImageSource)) {
    let start_load_image = &|src: &ImageSource| {
        static LOADING_IMAGES: StaticHashSet<ImageSource> = StaticHashSet::new();
        if LOADING_IMAGES.contains(src) {
            return;
        }
        LOADING_IMAGES.insert(src.clone());

        start_load_image(src);
    };

    let rendering_tree = input.rendering_tree;

    let mut ctx = { DrawContext::new(skia, start_load_image) };

    ctx.canvas().clear(Color::WHITE);
    rendering_tree.draw(&mut ctx);
    skia.surface().flush();
}
