use crate::*;

#[document]
struct EpisodeDoc {
    #[pk]
    id: String,
    name: String,
    created_at: SystemTime,
    scene_ids: Vec<String>,
}

#[document]
struct ProjectToEpisodeDoc {
    #[pk]
    project_id: String,
    #[sk]
    episode_id: String,
}

#[document]
struct EpisodeToProjectDoc {
    #[pk]
    episode_id: String,
    project_id: String,
}

#[document]
struct EpisodeEditingUserDoc {
    #[pk]
    episode_id: String,
    editing_user: Option<EditingUser>,
}

#[doc_part]
struct EditingUser {
    user_id: String,
    last_edit_time: SystemTime,
}
