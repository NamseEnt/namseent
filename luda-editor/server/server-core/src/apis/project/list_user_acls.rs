use crate::documents::*;
use futures::future::try_join_all;
use namui_type::Uuid;
use rpc::list_user_acls::{Error, Request, Response};

pub async fn list_user_acls(
    session: Option<SessionDocument>,
    Request {
        project_id,
        last_key,
    }: Request,
) -> rpc::list_user_acls::Result {
    if session.is_none() {
        return Err(Error::Unauthorized);
    }
    let session = session.unwrap();
    let project = ProjectDocumentGet { pk_id: project_id }
        .run()
        .await
        .map_err(|error| Error::Unknown(error.to_string()))?;

    let session_is_project_owner = session.user_id == project.owner_id;
    if !session_is_project_owner {
        return Err(Error::Unauthorized);
    }

    let user_in_project_acl_document_query = UserInProjectAclDocumentQuery {
        pk_project_id: project_id,
        last_sk: last_key.map(|last_key| UserInProjectAclDocumentSortKey { user_id: last_key }),
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;
    let user_acls = try_join_all(
        user_in_project_acl_document_query
            .documents
            .into_iter()
            .map(|document| async move {
                let user_name = UserDocumentGet {
                    pk_id: document.user_id,
                }
                .run()
                .await
                .map(|user| user.name)
                .unwrap_or("Unknown".to_string());
                Ok(rpc::list_user_acls::UserAcl {
                    user_id: document.user_id,
                    user_name,
                    permission: document.permission,
                })
            }),
    )
    .await?;
    let next_key = user_in_project_acl_document_query
        .next_page_key
        .map(|next_key| Uuid::try_parse(&next_key).unwrap());

    Ok(Response {
        user_acls,
        next_key,
    })
}
