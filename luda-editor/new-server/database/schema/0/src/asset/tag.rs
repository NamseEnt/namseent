use crate::*;

#[doc_part]
#[derive(Copy, PartialEq, Eq, Hash)]
#[archive_attr(derive(PartialEq, Eq, Hash))]
#[repr(u8)]
enum AssetSystemTag {
    // Sprite 0 ~ 39
    SpriteCharacter = 0,
    SpriteObject = 1,
    SpriteBackground = 2,
    // Audio 40 ~ 79
    AudioCharacter = 40,
    AudioProp = 41,
    AudioBackground = 42,
}

#[document]
struct AssetCustomTagDoc {
    #[pk]
    id: String,
    names: Translations,
}

#[doc_part]
#[derive(PartialEq, Eq, Hash)]
#[archive_attr(derive(PartialEq, Eq, Hash))]
enum AssetTag {
    System { tag: AssetSystemTag },
    Custom { id: String },
}
