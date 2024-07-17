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
