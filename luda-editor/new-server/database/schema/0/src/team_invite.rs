use crate::*;

#[document]
struct TeamInviteCodeDoc {
    #[id]
    code: String,
    team_id: u128,
    expiration_time: SystemTime,
}
