use crate::documents::*;
use rpc::edit_user_acl::{Error, Request, Response};

pub async fn edit_user_acl(
    session: Option<SessionDocument>,
    Request {
        project_id,
        user_id,
        permission,
    }: Request,
) -> rpc::edit_user_acl::Result {
    if session.is_none() {
        return Err(Error::Unauthorized);
    }
    let session = session.unwrap();

    let project = ProjectDocumentGet { pk_id: project_id }.run().await;
    if let Err(error) = project {
        return Err(Error::Unknown(error.to_string()));
    }
    let project = project.unwrap();

    if user_id == project.owner_id {
        return Err(Error::CannotSetOwnerPermission);
    }

    let is_session_has_permission = project.owner_id == session.user_id;

    if !is_session_has_permission {
        return Err(Error::Unauthorized);
    }

    match permission {
        Some(permission) => crate::dynamo_db()
            .transact()
            .put_item(UserInProjectAclDocument {
                project_id,
                user_id,
                permission,
            })
            .put_item(ProjectAclUserInDocument {
                user_id,
                project_id,
                permission,
            })
            .send()
            .await
            .map(|_| Response {})
            .map_err(|error| Error::Unknown(error.to_string())),
        None => crate::dynamo_db()
            .transact()
            .delete_item(UserInProjectAclDocumentDelete {
                pk_project_id: project.id,
                sk_user_id: user_id,
            })
            .delete_item(ProjectAclUserInDocumentDelete {
                pk_user_id: user_id,
                sk_project_id: project.id,
            })
            .send()
            .await
            .map(|_| Response {})
            .map_err(|error| Error::Unknown(error.to_string())),
    }
}
