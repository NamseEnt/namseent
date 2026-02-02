use crate::*;

#[derive(Clone, Default)]
pub struct AtlasStyle {
    pub paint: Option<Paint>,
}

#[derive(Clone)]
pub struct AtlasParam {
    pub atlas: Image,
    pub sprites: Vec<AtlasSprite>,
    pub style: AtlasStyle,
}

/// Creates a rendering tree node for drawing multiple sprites from an atlas image.
///
/// This is more efficient than drawing each sprite individually as it batches
/// all sprites into a single draw call.
pub fn atlas(
    AtlasParam {
        atlas,
        sprites,
        style,
    }: AtlasParam,
) -> RenderingTree {
    RenderingTree::Node(DrawCommand::Atlas {
        command: AtlasDrawCommand {
            atlas,
            sprites,
            paint: style.paint,
        }
        .into(),
    })
}

/// A builder for creating atlas rendering trees.
pub struct AtlasBuilder {
    atlas: Image,
    sprites: Vec<AtlasSprite>,
    paint: Option<Paint>,
}

impl AtlasBuilder {
    /// Creates a new atlas builder with the given atlas image.
    pub fn new(atlas: Image) -> Self {
        Self {
            atlas,
            sprites: Vec::new(),
            paint: None,
        }
    }

    /// Adds a sprite to the atlas.
    ///
    /// * `tex` - The source rectangle within the atlas image
    /// * `xform` - The transformation to apply to this sprite
    pub fn sprite(mut self, tex: Rect<Px>, xform: RSXform) -> Self {
        self.sprites.push(AtlasSprite { tex, xform });
        self
    }

    /// Adds a sprite at a specific position with optional scale and rotation.
    pub fn sprite_at(
        mut self,
        tex: Rect<Px>,
        position: Xy<Px>,
        scale: f32,
        rotation_radians: f32,
    ) -> Self {
        let anchor = Xy::new(tex.width() / 2.0, tex.height() / 2.0);
        let xform = RSXform::from_radians(scale, rotation_radians, position.x, position.y, anchor);
        self.sprites.push(AtlasSprite { tex, xform });
        self
    }

    /// Sets the paint to apply to all sprites.
    pub fn paint(mut self, paint: Paint) -> Self {
        self.paint = Some(paint);
        self
    }

    /// Builds the atlas rendering tree.
    pub fn build(self) -> RenderingTree {
        atlas(AtlasParam {
            atlas: self.atlas,
            sprites: self.sprites,
            style: AtlasStyle { paint: self.paint },
        })
    }
}

pub struct AtlasRender {
    pub atlas: Image,
    pub sprites: Vec<AtlasSprite>,
    pub paint: Option<Paint>,
}

impl Component for AtlasRender {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            atlas,
            sprites,
            paint,
        } = self;
        ctx.add(crate::atlas(crate::AtlasParam {
            atlas,
            sprites,
            style: AtlasStyle { paint },
        }));
    }
}
