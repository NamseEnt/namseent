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
