use crate::documents::*;

pub async fn is_project_editor(
    user_id: rpc::Uuid,
    project_id: rpc::Uuid,
) -> Result<bool, crate::storage::dynamo_db::GetItemError> {
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
