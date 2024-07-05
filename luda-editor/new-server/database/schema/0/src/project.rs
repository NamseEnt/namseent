use crate::*;

#[schema]
struct ProjectDoc {
    #[pk]
    id: String,
    team_id: String,
    name: String,
}

#[schema]
struct TeamToProjectDoc {
    #[pk]
    team_id: String,
    #[sk]
    project_id: String,
}

#[schema]
struct ProjectNameDoc {
    #[pk]
    team_id: String,
    #[pk]
    project_name: String,
}
