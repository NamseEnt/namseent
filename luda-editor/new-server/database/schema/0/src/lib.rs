use schema_macro::schema;

#[schema]
pub struct UserDoc {
    #[pk]
    pub id: String,
    pub name: String,
}

#[schema]
pub struct GoogleIdentityDoc {
    #[pk]
    pub sub: String,
    pub user_id: String,
}

#[schema]
pub struct SessionTokenDoc {
    #[pk]
    pub session_token: String,
    pub user_id: String,
}

#[schema]
pub struct TeamDoc {
    #[pk]
    pub id: String,
    pub name: String,
}

#[schema]
pub struct UserTeamDoc {
    #[pk]
    pub user_id: String,
    // #[sk]
    pub team_id: String,
}
