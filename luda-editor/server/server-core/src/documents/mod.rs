mod session;
mod user;

pub use session::*;
pub use user::*;

// RULES
// 1. Put simple documents here.
// 2. Create complex document modules in their own files.

#[document_macro::document]
pub struct IdentityDocument {
    #[pk]
    pub id: String,
    pub user_id: rpc::Uuid,
}
