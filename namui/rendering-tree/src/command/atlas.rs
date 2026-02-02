use crate::*;

/// A sprite entry for atlas drawing.
/// Each sprite has a source rectangle in the atlas image and a transform.
#[derive(Debug, PartialEq, Clone, Hash, Eq, State)]
pub struct AtlasSprite {
    /// The source rectangle within the atlas image
    pub tex: Rect<Px>,
    /// The transformation to apply to this sprite
    pub xform: RSXform,
}

/// Command for drawing multiple sprites from an atlas image in a single draw call.
/// This is more efficient than drawing each sprite individually.
#[derive(Debug, PartialEq, Clone, Hash, Eq, State)]
pub struct AtlasDrawCommand {
    /// The atlas image containing all sprites
    pub atlas: Image,
    /// The sprites to draw from the atlas
    pub sprites: Vec<AtlasSprite>,
    /// Optional paint to apply to all sprites
    pub paint: Option<Paint>,
}

impl AtlasDrawCommand {
    pub fn new(atlas: Image) -> Self {
        Self {
            atlas,
            sprites: Vec::new(),
            paint: None,
        }
    }

    pub fn with_sprites(mut self, sprites: Vec<AtlasSprite>) -> Self {
        self.sprites = sprites;
        self
    }

    pub fn with_paint(mut self, paint: Option<Paint>) -> Self {
        self.paint = paint;
        self
    }

    pub fn add_sprite(mut self, sprite: AtlasSprite) -> Self {
        self.sprites.push(sprite);
        self
    }
}
