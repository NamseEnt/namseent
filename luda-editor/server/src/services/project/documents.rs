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
