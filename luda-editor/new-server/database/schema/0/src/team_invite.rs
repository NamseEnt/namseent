use crate::*;

#[schema]
struct TeamInviteCodeDoc {
    #[pk]
    team_id: String,
    #[sk]
    code: String,
    expiration_time: SystemTime,
}

#[schema]
struct TeamInviteCodeToTeamDoc {
    #[pk]
    code: String,
    team_id: String,
}
