use crate::documents::*;
use futures::future::try_join_all;
use rpc::list_project_sequences::{Error, Request, Response};

pub async fn list_project_sequences(
    _session: Option<SessionDocument>,
    Request { project_id }: Request,
) -> rpc::list_project_sequences::Result {
    let project_sequence_query = ProjectSequenceDocumentQuery {
        pk_project_id: project_id,
        last_sk: None, // TODO
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    let sequence_name_and_ids = try_join_all(project_sequence_query.documents.into_iter().map(
        |project_sequence_document| async move {
            let sequence_index_document = SequenceIndexDocumentGet {
                pk_id: project_sequence_document.sequence_id,
            }
            .run()
            .await
            .map_err(|error| Error::Unknown(error.to_string()))?;

            let sequence_document = SequenceDocumentGet {
                pk_id: project_sequence_document.sequence_id,
                sk_index: sequence_index_document.index,
            }
            .run()
            .await
            .map_err(|error| Error::Unknown(error.to_string()))?;

            Ok(rpc::list_project_sequences::SequenceNameAndId {
                id: sequence_document.id,
                name: sequence_document.name,
            })
        },
    ))
    .await?;

    Ok(Response {
        sequence_name_and_ids,
    })
}
