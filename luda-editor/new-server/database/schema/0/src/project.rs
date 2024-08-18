use crate::*;

#[document]
struct ProjectDoc {
    #[pk]
    id: String,
    name: String,
}

#[document]
struct TeamToProjectDoc {
    #[pk]
    team_id: String,
    #[sk]
    project_id: String,
}

#[document]
struct ProjectToTeamDoc {
    #[pk]
    project_id: String,
    team_id: String,
}

#[document]
struct ProjectNameDoc {
    #[pk]
    team_id: String,
    #[pk]
    project_name: String,
}

#[document]
struct SpeakerDoc {
    #[pk]
    project_id: String,
    #[sk]
    speaker_id: String,
}

#[document]
struct SpeakerNameL10nDoc {
    #[pk]
    speaker_id: String,
    #[sk]
    language_code: String,
    text: String,
}
