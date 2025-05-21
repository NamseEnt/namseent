use crate::*;
use std::collections::HashSet;

#[document]
struct UserDoc {
    #[id]
    id: u128,
    name: String,
    team_ids: HashSet<u128>,
}

#[document]
struct GoogleIdentityDoc {
    #[id]
    sub: String,
    user_id: u128,
}

#[document]
struct SessionTokenDoc {
    #[id]
    session_token: u128,
    user_id: u128,
    expires_at: SystemTime,
}
