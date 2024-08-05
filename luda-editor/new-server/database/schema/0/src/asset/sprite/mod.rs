mod parts;

use crate::*;
pub use parts::*;
use std::collections::BTreeMap;

#[document]
struct SpriteDoc {
    #[pk]
    id: String,
    sprite: Sprite,
}

#[doc_part]
enum Sprite {
    Parts { sprite: PartsSprite },
    SingleImage { sprite: SingleImageSprite },
}

impl Sprite {
    pub fn name(&self) -> &str {
        match self {
            Sprite::Parts { sprite } => &sprite.name,
            Sprite::SingleImage { sprite } => &sprite.name,
        }
    }
}

#[doc_part]
struct PartsSprite {
    name: String,
    parts: BTreeMap<String, SpritePart>,
}

#[doc_part]
struct SingleImageSprite {
    id: String,
    name: String,
}
