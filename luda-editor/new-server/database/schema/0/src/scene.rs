use crate::*;
use std::collections::{HashMap, HashSet};

#[document]
struct SceneDoc {
    speaker_id: Option<String>,
    scene_sprites: Vec<SceneSprite>,
    /// `None` means the background should be black.
    background_sprite: Option<SceneSprite>,
    /// `None` means previous bgm should be continued.
    bgm: Option<SceneSound>,
    text_l10n: HashMap<LanguageCode, String>,
}
type LanguageCode = String;

#[doc_part]
struct SceneSprite {
    sprite_id: Option<String>,
    circumcircle: Circumcircle,
    /// - key: part id
    /// - value: part option ids
    ///
    /// For single image sprite, no entry should be in this map.
    part_option_selections: HashMap<String, HashSet<String>>,
}

#[doc_part]
struct SceneSound {
    sound_id: u128,
    volume: Percent,
}

#[doc_part]
struct Circumcircle {
    /// X and Y in Percent of the screen width and height
    xy: Xy<Percent>,
    /// If radius is 100%, the diagonal length of image will be the same with screen's diagonal length.
    radius: Percent,
}
