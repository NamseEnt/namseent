use crate::*;

#[document]
#[belongs_to(Project)]
struct EpisodeDoc {
    name: String,
    created_at: SystemTime,
    scene_ids: Vec<String>,
}

#[document]
struct EpisodeEditingUserDoc {
    #[id]
    episode_id: u128,
    editing_user: Option<EditingUser>,
}

#[doc_part]
struct EditingUser {
    user_id: u128,
    last_edit_time: SystemTime,
}
