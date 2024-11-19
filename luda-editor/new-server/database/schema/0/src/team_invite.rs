use crate::*;

#[document]
#[belongs_to(Team)]
struct TeamInviteCodeDoc {
    #[id]
    code: String,
    expiration_time: SystemTime,
}
