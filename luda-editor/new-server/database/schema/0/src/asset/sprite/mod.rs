use crate::*;

#[document]
struct SpriteDoc {
    #[pk]
    id: String,
    sprite: Sprite,
    tags: Vec<SpriteTag>,
}

#[doc_part]
enum SpriteTag {
    System { tag: SystemTag },
    Custom { id: String },
}

#[doc_part]
#[derive(Copy, PartialEq, Eq, Hash)]
#[archive_attr(derive(PartialEq, Eq, Hash))]
#[repr(u8)]
enum SystemTag {
    Character,
    Object,
    Background,
}

#[document]
struct SpriteTagDoc {
    #[pk]
    id: String,
    names: Translations,
}

#[doc_part]
struct Sprite {
    id: String,
    name: String,
    kind: SpriteKind,
}

#[doc_part]
enum SpriteKind {
    Parts,
    SingleImage,
}
