use crate::*;

#[document]
struct UserDoc {
    name: String,
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
    session_token: String,
    user_id: u128,
}
