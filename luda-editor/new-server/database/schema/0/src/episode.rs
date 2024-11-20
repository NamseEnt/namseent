use crate::*;
use std::collections::{BTreeMap, HashMap, HashSet};

#[document]
struct EpisodeDoc {
    #[id]
    id: u128,
    name: String,
    project_id: u128,
    created_at: SystemTime,
    scenes: BTreeMap<SceneId, Scene>,
    editing_user: Option<EditingUser>,
    speaker_slots: HashMap<UserId, HashSet<SpeakerId>>,
}

type UserId = u128;
type SpeakerId = u128;
type SceneId = u128;

#[doc_part]
struct EditingUser {
    user_id: u128,
    last_edit_time: SystemTime,
}
