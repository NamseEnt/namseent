use crate::MapCoordF32;
use namui::*;
use namui_prebuilt::simple_rect;
use rand::{Rng, thread_rng};

use super::{MAP_OUTSIDE_MARGIN_TILES, MAP_SIZE, TILE_PX_SIZE};

#[derive(Clone, Copy, State)]
pub struct MapDecoration {
    pub coord: MapCoordF32,
    pub scale: f32,
}

impl Component for &MapDecoration {
    fn render(self, ctx: &RenderCtx) {
        // Placeholder: dark green rect (2 tiles wide × 3 tiles tall) with scale variation.
        let tree_wh = Wh::new(
            TILE_PX_SIZE.width * 2.0 * self.scale,
            TILE_PX_SIZE.height * 3.0 * self.scale,
        );
        ctx.add(simple_rect(
            tree_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::from_u8(34, 85, 34, 200),
        ));
    }
}

pub fn generate_decorations() -> Vec<MapDecoration> {
    let mut rng = thread_rng();
    let mut decorations = vec![];

    let map_w = MAP_SIZE.width as f32;
    let map_h = MAP_SIZE.height as f32;
    let tree_w = 2.0f32;
    let tree_h = 3.0f32;
    let jitter_range = 1.0f32;
    let max_scale = 1.75f32;

    let left_x_min = -(MAP_OUTSIDE_MARGIN_TILES + tree_w);
    let left_x_max = 0.0f32;
    let right_x_min = map_w;
    let right_x_max = map_w + MAP_OUTSIDE_MARGIN_TILES;
    let top_y_min = -(MAP_OUTSIDE_MARGIN_TILES + tree_h);
    let top_y_max = 0.0f32;
    let bottom_y_min = map_h;
    let bottom_y_max = map_h + MAP_OUTSIDE_MARGIN_TILES;

    let max_attempts = 1024;
    let min_spacing = -0.25f32;

    for _ in 0..max_attempts {
        if decorations.len() >= 128 {
            break;
        }

        let scale = rng.gen_range(0.85..max_scale);
        let band = rng.gen_range(0..4);
        let (mut x, mut y) = match band {
            0 => (
                rng.gen_range(left_x_min..=left_x_max),
                rng.gen_range(top_y_min..=bottom_y_max),
            ),
            1 => (
                rng.gen_range(right_x_min..=right_x_max),
                rng.gen_range(top_y_min..=bottom_y_max),
            ),
            2 => (
                rng.gen_range(left_x_min..=right_x_max),
                rng.gen_range(top_y_min..=top_y_max),
            ),
            _ => (
                rng.gen_range(left_x_min..=right_x_max),
                rng.gen_range(bottom_y_min..=bottom_y_max),
            ),
        };

        x += rng.gen_range(-jitter_range..=jitter_range);
        y += rng.gen_range(-jitter_range..=jitter_range);

        if band == 0 {
            x = x.clamp(left_x_min, left_x_max);
        } else if band == 1 {
            x = x.clamp(right_x_min, right_x_max);
        }
        if band == 2 {
            y = y.clamp(top_y_min, top_y_max);
        } else if band == 3 {
            y = y.clamp(bottom_y_min, bottom_y_max);
        }

        let scaled_tree_w = tree_w * scale;
        let scaled_tree_h = tree_h * scale;

        let inside_map =
            x + scaled_tree_w > 0.0 && x < map_w && y + scaled_tree_h > 0.0 && y < map_h;
        if inside_map {
            continue;
        }

        let new_left = x;
        let new_top = y;
        let new_right = x + scaled_tree_w;
        let new_bottom = y + scaled_tree_h;

        let overlaps_existing = decorations.iter().any(|existing: &MapDecoration| {
            let existing_w = tree_w * existing.scale;
            let existing_h = tree_h * existing.scale;
            let existing_left = existing.coord.x;
            let existing_top = existing.coord.y;
            let existing_right = existing_left + existing_w;
            let existing_bottom = existing_top + existing_h;

            new_left < existing_right + min_spacing
                && new_right + min_spacing > existing_left
                && new_top < existing_bottom + min_spacing
                && new_bottom + min_spacing > existing_top
        });
        if overlaps_existing {
            continue;
        }

        decorations.push(MapDecoration {
            coord: MapCoordF32::new(x, y),
            scale,
        });
    }

    decorations
}

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
