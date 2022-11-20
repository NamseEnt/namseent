use crate::{
    app::game::{tile, Tile},
    component::{Sprite, SpriteAnimation},
};
use namui::prelude::*;

const VISUAL_WIDTH: Tile = tile(3.0);
const VISUAL_HEIGHT: Tile = tile(4.0);
const VISUAL_OFFSET_X: Tile = tile(-1.5);
const VISUAL_OFFSET_Y: Tile = tile(-2.5);

pub fn walking_sprite_animation(started_at: Time) -> SpriteAnimation {
    SpriteAnimation::new(
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
        started_at,
    )
}

pub fn standing_sprite() -> Sprite {
    Sprite {
        image_url: Url::parse("bundle:image/character/character_0.png").unwrap(),
        visual_rect: Rect::Xywh {
            x: VISUAL_OFFSET_X,
            y: VISUAL_OFFSET_Y,
            width: VISUAL_WIDTH,
            height: VISUAL_HEIGHT,
        },
    }
}
