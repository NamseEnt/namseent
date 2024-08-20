use crate::*;
use rkyv::*;

#[doc_part]
/// This struct just stores the options to on/off the parts of a sprite.
struct SpritePart {
    name: String,
    is_single_select: bool,
    part_options: Vec<SpritePartOption>,
}

#[doc_part]
struct SpritePartOption {
    name: String,
}
