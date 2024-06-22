mod level;
mod piece;
// mod playground;
mod level_select;
mod solution_board;

use level_select::LevelSelect;
use namui::*;
use namui_prebuilt::simple_rect;

const BACKGROUND: &str = "bundle:background.jpg";

pub fn main() {
    namui::start(render)
}

fn render(ctx: &RenderCtx) {
    let wh = screen::size().into_type::<Px>();
    let background = ctx.image(BACKGROUND);

    ctx.compose(|ctx| {
        ctx.add(LevelSelect);
    });

    ctx.compose(|ctx| {
        let Some(Ok(image)) = background.as_ref() else {
            ctx.add(simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::BLACK));
            return;
        };
        ctx.add(ImageDrawCommand {
            rect: wh.to_rect(),
            image: image.clone(),
            fit: ImageFit::Cover,
            paint: None,
        });
    });
}
