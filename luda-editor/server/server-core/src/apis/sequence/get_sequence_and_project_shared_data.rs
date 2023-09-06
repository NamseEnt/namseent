use crate::documents::*;
use rpc::get_sequence_and_project_shared_data::{Error, Request, Response};

pub async fn get_sequence_and_project_shared_data(
    _session: Option<SessionDocument>,
    Request { sequence_id }: Request,
) -> rpc::get_sequence_and_project_shared_data::Result {
    let sequence_index_document = SequenceIndexDocumentGet { pk_id: sequence_id }
        .run()
        .await
        .map_err(|error| match error {
            crate::storage::dynamo_db::GetItemError::NotFound => Error::SequenceNotFound,
            _ => Error::Unknown(error.to_string()),
        })?;

    let sequence_document = SequenceDocumentGet {
        pk_id: sequence_id,
        sk_index: sequence_index_document.index,
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    let project = ProjectDocumentGet {
        pk_id: sequence_document.project_id,
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    let cut_futures = sequence_document.cuts.iter().map(|cut_index| async move {
        SequenceCutDocumentGet {
            pk_sequence_id: sequence_id,
            pk_cut_id: cut_index.cut_id,
            sk_cut_index: cut_index.index,
        }
        .run()
        .await
        .map_err(|error| Error::Unknown(error.to_string()))
        .map(|document| document.cut)
    });

    let cuts = futures::future::try_join_all(cut_futures).await?;

    let sequence = rpc::data::Sequence {
        id: sequence_document.id,
        name: sequence_document.name,
        cuts,
    };

    Ok(Response {
        sequence,
        project_shared_data_json: project.shared_data_json,
    })
}
