use super::is_project_editor;
use crate::documents::*;

pub async fn check_session_project_editor<E>(
    session: Option<SessionDocument>,
    project_id: rpc::Uuid,
    on_unauthorized: impl FnOnce() -> E,
    on_unknown: impl FnOnce(String) -> E,
) -> Result<(), E> {
    let Some(session) = session else {
        return Err(on_unauthorized());
    };
    match is_project_editor(session.user_id, project_id).await {
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
