use crate::{
    asset,
    game_state::{TILE_PX_SIZE, TRAVEL_POINTS},
};
use namui::*;

pub fn render_route_flag(ctx: &RenderCtx) {
    for (index, coord) in TRAVEL_POINTS.iter().enumerate() {
        let xy = Xy::new(
            coord.x.as_f32() * TILE_PX_SIZE.width.as_f32(),
            (coord.y.as_f32() - 1.125) * TILE_PX_SIZE.height.as_f32(),
        )
        .map(px);

        let Some(image) = get_flag_image(index) else {
            continue;
        };
        let image_wh = image.info().wh();
        ctx.translate(xy).add(namui::image(ImageParam {
            rect: Rect::from_xy_wh(Xy::zero(), image_wh),
            image,
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: None,
            },
        }));
    }
}

fn get_flag_image(index: usize) -> Option<Image> {
    match index {
        1 => Some(asset::image::route::FLAG_1),
        2 => Some(asset::image::route::FLAG_2),
        3 => Some(asset::image::route::FLAG_3),
        4 => Some(asset::image::route::FLAG_4),
        5 => Some(asset::image::route::FLAG_5),
        _ => None,
    }
}
