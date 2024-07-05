use crate::*;

#[schema]
struct TeamDoc {
    #[pk]
    id: String,
    name: String,
}

#[schema]
struct UserToTeamDoc {
    #[pk]
    user_id: String,
    #[sk]
    team_id: String,
}

#[schema]
struct TeamNameToTeamIdDoc {
    #[pk]
    team_name: String,
    team_id: String,
}
