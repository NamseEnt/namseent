use crate::*;

#[document]
struct TeamInviteCodeDoc {
    #[pk]
    team_id: String,
    #[sk]
    code: String,
    expiration_time: SystemTime,
}

#[document]
struct TeamInviteCodeToTeamDoc {
    #[pk]
    code: String,
    team_id: String,
}
