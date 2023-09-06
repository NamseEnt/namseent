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

    let editable_projects = try_join_all(owner_project_query.documents.into_iter().map(
        |owner_project_document| async move {
            match (ProjectDocumentGet {
                pk_id: owner_project_document.project_id,
            })
            .run()
            .await
            {
                Ok(project) => Ok(rpc::list_editable_projects::EditableProject {
                    id: owner_project_document.project_id,
                    name: project.name,
                }),
                Err(error) => Err(Error::Unknown(error.to_string())),
            }
        },
    ))
    .await;
    if let Err(error) = editable_projects {
        return Err(Error::Unknown(error.to_string()));
    }
    let editable_projects = editable_projects.unwrap();

    Ok(Response {
        projects: editable_projects,
    })
}
