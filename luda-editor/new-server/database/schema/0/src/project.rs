use crate::*;
use std::collections::HashMap;

#[document]
#[belongs_to(Team)]
struct ProjectDoc {
    name: String,
}

#[document]
#[belongs_to(Project)]
struct SpeakerDoc {
    name_l10n: HashMap<LanguageCode, String>,
}
type LanguageCode = String;
