use crate::*;

#[document]
struct UserDoc {
    #[pk]
    id: String,
    name: String,
}

#[document]
struct GoogleIdentityDoc {
    #[pk]
    sub: String,
    user_id: String,
}

#[document]
struct SessionTokenDoc {
    #[pk]
    session_token: String,
    user_id: String,
}
