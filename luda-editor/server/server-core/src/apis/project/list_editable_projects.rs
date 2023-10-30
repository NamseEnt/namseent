use crate::documents::*;
use futures::future::try_join_all;
use rpc::list_editable_projects::{Error, Request, Response};

pub async fn list_editable_projects(
    session: Option<SessionDocument>,
    Request { start_after: _ }: Request,
) -> rpc::list_editable_projects::Result {
    if session.is_none() {
        return Err(Error::Unauthorized);
    }
    let session = session.unwrap();
    let owner_project_query = OwnerProjectDocumentQuery {
        pk_owner_id: session.user_id,
        last_sk: None, // TODO
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    let project_acl_user_query = ProjectAclUserInDocumentQuery {
        pk_user_id: session.user_id,
        last_sk: None, // TODO
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    let editable_project_ids = owner_project_query
        .documents
        .into_iter()
        .map(|owner_project_document| owner_project_document.project_id)
        .chain(
            project_acl_user_query
                .documents
                .into_iter()
                .map(|project_acl_user_document| project_acl_user_document.project_id),
        );

    let editable_projects = try_join_all(editable_project_ids.map(|project_id| async move {
        (ProjectDocumentGet { pk_id: project_id })
            .run()
            .await
            .map(|project| rpc::list_editable_projects::EditableProject {
                id: project_id,
                name: project.name,
            })
            .map_err(|error| Error::Unknown(error.to_string()))
    }))
    .await;
    if let Err(error) = editable_projects {
        return Err(Error::Unknown(error.to_string()));
    }
    let editable_projects = editable_projects.unwrap();

    Ok(Response {
        projects: editable_projects,
    })
}
