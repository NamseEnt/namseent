use crate::app::game::*;
use crate::component::*;
use namui::prelude::*;

const VISUAL_WIDTH: Tile = tile(1.0);
const VISUAL_HEIGHT: Tile = tile(1.0);
const VISUAL_OFFSET_X: Tile = tile(-0.5);
const VISUAL_OFFSET_Y: Tile = tile(-0.5);

pub fn new_floor(app: &mut crate::ecs::App, positions: Vec<Xy<Tile>>) -> &mut crate::ecs::Entity {
    let min_x = positions
        .iter()
        .map(|p| p.x)
        .reduce(|a, b| a.min(b))
        .unwrap();
    let min_y = positions
        .iter()
        .map(|p| p.y)
        .reduce(|a, b| a.min(b))
        .unwrap();
    let max_x = positions
        .iter()
        .map(|p| p.x)
        .reduce(|a, b| a.max(b))
        .unwrap();
    let max_y = positions
        .iter()
        .map(|p| p.y)
        .reduce(|a, b| a.max(b))
        .unwrap();

    app.new_entity()
        .add_component(Positioner::new())
        .add_component(Renderer::new(
            -1,
            Rect::Xywh {
                x: VISUAL_OFFSET_X + min_x,
                y: VISUAL_OFFSET_Y + min_y,
                width: VISUAL_WIDTH + max_x - min_x,
                height: VISUAL_HEIGHT + max_y - min_y,
            },
            RenderType::Floor {
                positions,
                visual_offset_rect: Rect::Xywh {
                    x: VISUAL_OFFSET_X,
                    y: VISUAL_OFFSET_Y,
                    width: VISUAL_WIDTH,
                    height: VISUAL_HEIGHT,
                },
            },
        ))
}
