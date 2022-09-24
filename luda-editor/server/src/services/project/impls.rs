use super::*;

impl ProjectService {
    pub async fn is_project_editor(
        &self,
        user_id: rpc::Uuid,
        project_id: rpc::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let project = crate::dynamo_db()
            .get_item::<ProjectDocument>(project_id, Option::<String>::None)
            .await?;

        if project.owner_id == user_id {
            return Ok(true);
        }

        let acl = crate::dynamo_db()
            .get_item::<ProjectAclUserInDocument>(user_id, Some(project_id.to_string()))
            .await?;

        match acl.permission {
            rpc::types::ProjectAclUserPermission::Editor => Ok(true),
        }
    }
}
