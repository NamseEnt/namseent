use crate::*;

#[document]
struct TeamInviteCodeDoc {
    #[id]
    code: u128,
    team_id: u128,
    expiration_time: SystemTime,
}
