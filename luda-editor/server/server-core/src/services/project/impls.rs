use super::*;

impl ProjectService {
    pub async fn is_project_editor(
        &self,
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
    pub async fn check_session_project_editor<E>(
        &self,
        session: Option<SessionDocument>,
        project_id: rpc::Uuid,
        on_unauthorized: impl FnOnce() -> E,
        on_unknown: impl FnOnce(String) -> E,
    ) -> Result<(), E> {
        let Some(session) = session else {
            return Err(on_unauthorized());
        };
        match self.is_project_editor(session.user_id, project_id).await {
            Ok(is_project_editor) => {
                if is_project_editor {
                    Ok(())
                } else {
                    Err(on_unauthorized())
                }
            }
            Err(err) => match err {
                crate::storage::dynamo_db::GetItemError::NotFound => Err(on_unauthorized()),
                crate::storage::dynamo_db::GetItemError::DeserializeFailed(err)
                | crate::storage::dynamo_db::GetItemError::Unknown(err) => Err(on_unknown(err)),
            },
        }
    }
}
