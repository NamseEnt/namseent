use crate::*;
use rkyv::*;

#[doc_part]
struct SpritePart {
    id: String,
    name: String,
    kind: SpritePartKind,
    blend_mode: BlendMode,
}

#[doc_part]
enum SpritePartKind {
    Fixed { image: SpritePartOption },
    SingleSelect { images: Vec<SpritePartOption> },
    MultiSelect { images: Vec<SpritePartOption> },
}

#[doc_part]
struct SpritePartOption {
    id: String,
    name: String,
    blend_mode: BlendMode,
    dest_rect: Rect<Px>,
}

#[derive(Clone, Copy)]
#[doc_part]
enum BlendMode {
    PassThrough,
    Normal,
    Dissolve,
    Darken,
    Multiply,
    ColorBurn,
    LinearBurn,
    DarkerColor,
    Lighten,
    Screen,
    ColorDodge,
    LinearDodge,
    LighterColor,
    Overlay,
    SoftLight,
    HardLight,
    VividLight,
    LinearLight,
    PinLight,
    HardMix,
    Difference,
    Exclusion,
    Subtract,
    Divide,
    Hue,
    Saturation,
    Color,
    Luminosity,
}
