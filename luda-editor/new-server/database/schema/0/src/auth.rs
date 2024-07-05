use crate::*;

#[schema]
struct UserDoc {
    #[pk]
    id: String,
    name: String,
}

#[schema]
struct GoogleIdentityDoc {
    #[pk]
    sub: String,
    user_id: String,
}

#[schema]
struct SessionTokenDoc {
    #[pk]
    session_token: String,
    user_id: String,
}
