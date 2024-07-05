use document::*;
use std::time::SystemTime;

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
