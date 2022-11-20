use crate::app::game::{known_id::object::PLAYER_CHARACTER, *};
use crate::component::*;
use namui::prelude::*;

const VISUAL_WIDTH: Tile = tile(3.0);
const VISUAL_HEIGHT: Tile = tile(4.0);
const VISUAL_OFFSET_X: Tile = tile(-1.5);
const VISUAL_OFFSET_Y: Tile = tile(-2.5);

pub fn new_player(app: &mut crate::ecs::App, xy: Xy<Tile>) -> &mut crate::ecs::Entity {
    app.new_entity_with_id(PLAYER_CHARACTER)
        .add_component(Positioner::new_with_xy(xy))
        .add_component(Collider::from_circle(Xy::zero(), tile(1.5)))
        .add_component(PlayerCharacter {
            heading: Heading::Left,
        })
        .add_component(Renderer::new(
            0,
            RenderType::SpriteAnimation(SpriteAnimation::new(
                vec![
                    Sprite {
                        image_url: Url::parse("bundle:image/character/character_0.png").unwrap(),
                        visual_rect: Rect::Xywh {
                            x: VISUAL_OFFSET_X,
                            y: VISUAL_OFFSET_Y,
                            width: VISUAL_WIDTH,
                            height: VISUAL_HEIGHT,
                        },
                    },
                    Sprite {
                        image_url: Url::parse("bundle:image/character/character_1.png").unwrap(),
                        visual_rect: Rect::Xywh {
                            x: VISUAL_OFFSET_X,
                            y: VISUAL_OFFSET_Y,
                            width: VISUAL_WIDTH,
                            height: VISUAL_HEIGHT,
                        },
                    },
                ],
                100.ms(),
                0.ms(),
            )),
        ))
        .add_component(Mover::new())
}
