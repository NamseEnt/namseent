use crate::app::game::*;
use crate::component::*;
use namui::prelude::*;

const VISUAL_WIDTH: Tile = tile(1.0);
const VISUAL_HEIGHT: Tile = tile(1.0);
const VISUAL_OFFSET_X: Tile = tile(-0.5);
const VISUAL_OFFSET_Y: Tile = tile(-0.5);

pub fn new_floor(app: &mut crate::ecs::App, positions: Vec<Xy<Tile>>) -> &mut crate::ecs::Entity {
    let sprites = positions
        .into_iter()
        .map(|position| Sprite {
            visual_rect: Rect::Xywh {
                x: position.x + VISUAL_OFFSET_X,
                y: position.y + VISUAL_OFFSET_Y,
                width: VISUAL_WIDTH,
                height: VISUAL_HEIGHT,
            },
            image_url: Url::parse("bundle:image/floor.png").unwrap(),
        })
        .collect();
    let mut sprite_batch = SpriteBatch::new(sprites);
    let center_xy = sprite_batch.visual_rect.center();
    sprite_batch.translate(Xy {
        x: -center_xy.x,
        y: -center_xy.y,
    });

    app.new_entity()
        .add_component(Positioner::new_with_xy(center_xy))
        .add_component(Renderer::new(-1, RenderType::SpriteBatch(sprite_batch)))
}
