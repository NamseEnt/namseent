use crate::storage::dynamo_db::Document;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectDocument {
    pub id: String,
    pub owner_id: String,
    pub name: String,
}

impl Document for ProjectDocument {
    fn partition_key_prefix() -> &'static str {
        "project"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.id.clone()
    }

    fn sort_key(&self) -> Option<&str> {
        None
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OwnerProjectDocument {
    pub owner_id: String,
    pub project_id: String,
}

impl Document for OwnerProjectDocument {
    fn partition_key_prefix() -> &'static str {
        "owner_project"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.owner_id.clone()
    }

    fn sort_key(&self) -> Option<&str> {
        Some(&self.project_id)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserInProjectAclDocument {
    pub user_id: String,
    pub project_id: String,
    pub permission: rpc::types::ProjectAclUserPermission,
}

impl Document for UserInProjectAclDocument {
    fn partition_key_prefix() -> &'static str {
        "user_in_project_acl"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.project_id.clone()
    }

    fn sort_key(&self) -> Option<&str> {
        Some(&self.user_id)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProjectAclUserInDocument {
    pub user_id: String,
    pub project_id: String,
    pub permission: rpc::types::ProjectAclUserPermission,
}

impl Document for ProjectAclUserInDocument {
    fn partition_key_prefix() -> &'static str {
        "project_acl_user_in"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.user_id.clone()
    }

    fn sort_key(&self) -> Option<&str> {
        Some(&self.project_id)
    }
}
