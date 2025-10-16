use crate::MapCoordF32;
use namui::*;
use namui_prebuilt::simple_rect;
use rand::{Rng, thread_rng};

#[derive(Clone, Copy, State)]
pub struct Background {
    pub coord: MapCoordF32,
    kind: BackgroundKind,
    flip_horizontally: bool,
}
impl Component for &Background {
    fn render(self, ctx: &RenderCtx) {
        let image = self.kind.image();
        let image_wh = image.info().wh();
        ctx.compose(|mut ctx| {
            if self.flip_horizontally {
                ctx = ctx
                    .scale(Xy::new(-1.0, 1.0))
                    .translate((-image_wh.width, 0.px()));
            }

            ctx.add(namui::image(ImageParam {
                rect: Rect::from_xy_wh(Xy::zero(), image.info().wh()),
                image,
                style: ImageStyle {
                    fit: ImageFit::None,
                    paint: None,
                },
            }));
        });

        ctx.add(simple_rect(
            image_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::TRANSPARENT,
        ));
    }
}

#[derive(Clone, Copy, State)]
pub enum BackgroundKind {
    Tile0,
    Tile1,
    Tile2,
    Tile3,
}
impl BackgroundKind {
    pub fn image(self) -> Image {
        match self {
            BackgroundKind::Tile0 => crate::asset::image::background::TILE0,
            BackgroundKind::Tile1 => crate::asset::image::background::TILE1,
            BackgroundKind::Tile2 => crate::asset::image::background::TILE2,
            BackgroundKind::Tile3 => crate::asset::image::background::TILE3,
        }
    }
}

pub fn generate_backgrounds() -> Vec<Background> {
    let mut rng = thread_rng();

    let mut backgrounds = vec![];

    for y in (-6)..12 {
        for x in (-6)..12 {
            let coord = MapCoordF32::new(x as f32 * 8.0, y as f32 * 8.0);

            backgrounds.push(Background {
                coord,
                kind: match rng.gen_range(0..=3) {
                    0 => BackgroundKind::Tile0,
                    1 => BackgroundKind::Tile1,
                    2 => BackgroundKind::Tile2,
                    _ => BackgroundKind::Tile3,
                },
                flip_horizontally: rng.r#gen(),
            });
        }
    }
    backgrounds
}
