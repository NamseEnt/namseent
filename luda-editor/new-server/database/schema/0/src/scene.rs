use crate::*;
use std::collections::{HashMap, HashSet};

#[document]
struct SceneDoc {
    #[pk]
    id: String,
    speaker_id: Option<String>,
    scene_sprites: Vec<SceneSprite>,
    /// `None` means the background should be black.
    background_sprite: Option<SceneSprite>,
    /// `None` means previous bgm should be continued.
    bgm: Option<SceneSound>,
}

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
    sound_id: String,
    volume: Percent,
}

#[doc_part]
struct Circumcircle {
    /// X and Y in Percent of the screen width and height
    xy: Xy<Percent>,
    /// If radius is 100%, the diagonal length of image will be the same with screen's diagonal length.
    radius: Percent,
}

#[document]
struct SceneTextL10nDoc {
    #[pk]
    scene_id: String,
    #[sk]
    language_code: String,
    text: String,
}
