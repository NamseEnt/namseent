use crate::storage::dynamo_db::Document;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectDocument {
    pub id: rpc::Uuid,
    pub owner_id: rpc::Uuid,
    pub name: String,
    pub shared_data_json: String,
}

impl Document for ProjectDocument {
    fn partition_key_prefix() -> &'static str {
        "project"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.id.to_string()
    }

    fn sort_key(&self) -> Option<String> {
        None
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnerProjectDocument {
    pub owner_id: rpc::Uuid,
    pub project_id: rpc::Uuid,
}

impl Document for OwnerProjectDocument {
    fn partition_key_prefix() -> &'static str {
        "owner_project"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.owner_id.to_string()
    }

    fn sort_key(&self) -> Option<String> {
        Some(self.project_id.to_string())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserInProjectAclDocument {
    pub user_id: rpc::Uuid,
    pub project_id: rpc::Uuid,
    pub permission: rpc::types::ProjectAclUserPermission,
}

impl Document for UserInProjectAclDocument {
    fn partition_key_prefix() -> &'static str {
        "user_in_project_acl"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.project_id.to_string()
    }

    fn sort_key(&self) -> Option<String> {
        Some(self.user_id.to_string())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProjectAclUserInDocument {
    pub user_id: rpc::Uuid,
    pub project_id: rpc::Uuid,
    pub permission: rpc::types::ProjectAclUserPermission,
}

impl Document for ProjectAclUserInDocument {
    fn partition_key_prefix() -> &'static str {
        "project_acl_user_in"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.user_id.to_string()
    }

    fn sort_key(&self) -> Option<String> {
        Some(self.project_id.to_string())
    }
}
