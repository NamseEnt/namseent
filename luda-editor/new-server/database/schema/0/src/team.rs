use crate::*;

#[document]
struct TeamDoc {
    #[pk]
    id: String,
    name: String,
}

#[document]
struct UserToTeamDoc {
    #[pk]
    user_id: String,
    #[sk]
    team_id: String,
}

#[document]
struct TeamNameToTeamIdDoc {
    #[pk]
    team_name: String,
    team_id: String,
}
