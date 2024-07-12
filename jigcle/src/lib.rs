mod level;
mod level_select;
mod piece;
mod playground;
mod solution_board;

use level_select::LevelSelect;
use namui::*;
use playground::{Playground, PLAYING_LEVEL_ATOM};

const BACKGROUND: &str = "background.jpg";

pub fn main() {
    namui::start(render)
}

fn render(ctx: &RenderCtx) {
    let (playing_level, _) = ctx.init_atom(&PLAYING_LEVEL_ATOM, || None);
    let is_playing = playing_level.is_some();

    let wh = screen::size().into_type::<Px>();
    let background = ImageSource::ResourceLocation {
        resource_location: ResourceLocation::bundle(BACKGROUND),
    };

    ctx.compose(|ctx| {
        if !is_playing {
            ctx.add(LevelSelect);
        }
    });

    ctx.compose(|ctx| {
        if is_playing {
            ctx.add(Playground);
        }
    });

    ctx.add(ImageRender {
        rect: wh.to_rect(),
        source: background,
        fit: ImageFit::Cover,
        paint: None,
    });
}
