use super::*;

impl ProjectService {
    pub async fn is_project_editor(
        &self,
        user_id: rpc::Uuid,
        project_id: rpc::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let project = ProjectDocumentGet { pk_id: project_id }.run().await?;

        if project.owner_id == user_id {
            return Ok(true);
        }

        let acl = ProjectAclUserInDocumentGet {
            pk_user_id: user_id,
            sk_project_id: project_id,
        }
        .run()
        .await?;

        match acl.permission {
            rpc::types::ProjectAclUserPermission::Editor => Ok(true),
        }
    }
}
