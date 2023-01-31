mod user;

pub use user::*;

#[document_macro::document]
pub struct IdentityDocument {
    #[pk]
    pub id: String,
    pub user_id: rpc::Uuid,
}
