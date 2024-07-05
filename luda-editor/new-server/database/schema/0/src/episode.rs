use crate::*;

#[schema]
struct EpisodeDoc {
    #[pk]
    id: String,
    name: String,
    created_at: SystemTime,
}

#[schema]
struct ProjectToEpisodeDoc {
    #[pk]
    project_id: String,
    #[sk]
    episode_id: String,
}
