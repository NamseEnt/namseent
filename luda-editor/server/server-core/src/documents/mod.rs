mod memo;
mod project_image;
mod sequence;
mod session;
mod user;

pub use memo::*;
pub use project_image::*;
pub use sequence::*;
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

#[document_macro::document]
pub struct ProjectDocument {
    #[pk]
    pub id: rpc::Uuid,
    pub owner_id: rpc::Uuid,
    pub name: String,
    pub shared_data_json: String,
}

#[document_macro::document]
pub struct OwnerProjectDocument {
    #[pk]
    pub owner_id: rpc::Uuid,
    #[sk]
    pub project_id: rpc::Uuid,
}

#[document_macro::document]
pub struct UserInProjectAclDocument {
    #[pk]
    pub project_id: rpc::Uuid,
    #[sk]
    pub user_id: rpc::Uuid,
    pub permission: rpc::types::ProjectAclUserPermission,
}

#[document_macro::document]
pub struct ProjectAclUserInDocument {
    #[pk]
    pub user_id: rpc::Uuid,
    #[sk]
    pub project_id: rpc::Uuid,
    pub permission: rpc::types::ProjectAclUserPermission,
}

#[document_macro::document]
pub struct CgDocument {
    #[pk]
    pub project_id: rpc::Uuid,
    #[sk]
    pub cg_id: rpc::Uuid,
    pub cg_file: rpc::data::CgFile,
}
