use crate::*;
use rkyv::*;

#[doc_part]
struct SpritePart {
    id: String,
    name: String,
    kind: SpritePartKind,
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
    dest_rect: Rect<Px>,
}
