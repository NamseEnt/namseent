mod documents;

use super::project::documents::*;
use crate::session::SessionDocument;
use anyhow::Result;
use documents::*;
use futures::future::try_join_all;
use rpc::data::Sequence;

#[derive(Debug)]
pub struct SequenceService {}

impl SequenceService {
    pub fn new() -> Self {
        SequenceService {}
    }
}

impl rpc::SequenceService<SessionDocument> for SequenceService {
    fn list_project_sequences<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        req: rpc::list_project_sequences::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::list_project_sequences::Result> + Send>,
    > {
        Box::pin(async move {
            let project_sequence_documents = ProjectSequenceDocumentQuery {
                pk_project_id: req.project_id,
            }
            .run()
            .await;
            if let Err(error) = project_sequence_documents {
                return Err(rpc::list_project_sequences::Error::Unknown(
                    error.to_string(),
                ));
            }
            let project_sequence_documents = project_sequence_documents.unwrap();

            let sequence_name_and_ids = try_join_all(project_sequence_documents.into_iter().map(
                |project_sequence_document| async move {
                    match (SequenceDocumentGet {
                        pk_id: project_sequence_document.sequence_id,
                    })
                    .run()
                    .await
                    {
                        Ok(sequence) => Ok(rpc::list_project_sequences::SequenceNameAndId {
                            id: sequence.id,
                            name: sequence.name,
                        }),
                        Err(error) => Err(rpc::list_project_sequences::Error::Unknown(
                            error.to_string(),
                        )),
                    }
                },
            ))
            .await;
            if let Err(error) = sequence_name_and_ids {
                return Err(rpc::list_project_sequences::Error::Unknown(
                    error.to_string(),
                ));
            }
            let sequence_name_and_ids = sequence_name_and_ids.unwrap();

            Ok(rpc::list_project_sequences::Response {
                sequence_name_and_ids,
            })
        })
    }

    fn create_sequence<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::create_sequence::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::create_sequence::Result> + Send>,
    > {
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::create_sequence::Error::Unauthorized);
            }
            let session = session.unwrap();
            let is_project_editor = crate::services()
                .project_service
                .is_project_editor(session.user_id, req.project_id)
                .await
                .map_err(|error| rpc::create_sequence::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::create_sequence::Error::Unauthorized);
            }

            let sequence_id = rpc::Uuid::new_v4();
            let sequence = Sequence::new(sequence_id.clone(), req.name.clone());

            crate::dynamo_db()
                .transact()
                .create_item(SequenceDocument {
                    id: sequence_id.clone(),
                    project_id: req.project_id.clone(),
                    name: req.name,
                    json_brotli: compress_serde_using_brotli(&sequence)
                        .map_err(|error| rpc::create_sequence::Error::Unknown(error.to_string()))?,
                    last_modified: None,
                })
                .create_item(ProjectSequenceDocument {
                    project_id: req.project_id,
                    sequence_id,
                })
                .send()
                .await
                .map_err(|error| rpc::create_sequence::Error::Unknown(error.to_string()))?;

            Ok(rpc::create_sequence::Response {})
        })
    }

    fn update_server_sequence<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::update_server_sequence::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<
                    Output = Result<
                        rpc::update_server_sequence::Response,
                        rpc::update_server_sequence::Error,
                    >,
                >
                + Send,
        >,
    > {
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::update_server_sequence::Error::Unauthorized);
            }
            let session = session.unwrap();

            SequenceDocumentUpdate {
                pk_id: req.sequence_id,
                update: move |mut document: SequenceDocument| async move {
                    let is_project_editor = crate::services()
                        .project_service
                        .is_project_editor(session.user_id, document.project_id)
                        .await
                        .map_err(|error| {
                            log::warn!("Error: {}", error);
                            rpc::update_server_sequence::Error::Unknown(error.to_string())
                        })?;

                    if !is_project_editor {
                        return Err(rpc::update_server_sequence::Error::Unauthorized);
                    }

                    apply_sequence_patch(&mut document, &req.patch).map_err(|error| {
                        rpc::update_server_sequence::Error::Unknown(error.to_string())
                    })?;

                    Ok(document)
                },
            }
            .run()
            .await
            .map_err(|error| match error {
                crate::storage::dynamo_db::UpdateItemError::Canceled(error) => error,
                crate::storage::dynamo_db::UpdateItemError::NotFound
                | crate::storage::dynamo_db::UpdateItemError::SerializationFailed(_)
                | crate::storage::dynamo_db::UpdateItemError::Conflict
                | crate::storage::dynamo_db::UpdateItemError::Unknown(_) => {
                    log::warn!("Error: {}", error);
                    rpc::update_server_sequence::Error::Unknown(error.to_string())
                }
            })?;

            Ok(rpc::update_server_sequence::Response {})
        })
    }

    fn update_client_sequence<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        req: rpc::update_client_sequence::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::update_client_sequence::Result> + Send>,
    > {
        Box::pin(async move {
            let document = SequenceDocumentGet {
                pk_id: req.sequence_id,
            }
            .run()
            .await
            .map_err(|error| rpc::update_client_sequence::Error::Unknown(error.to_string()))?;

            let sequence_json = document
                .sequence()
                .map_err(|err| rpc::update_client_sequence::Error::Unknown(err.to_string()))?;
            let patch = rpc::json_patch::diff(&req.sequence_json, &sequence_json);

            Ok(rpc::update_client_sequence::Response { patch })
        })
    }

    fn get_sequence_and_project_shared_data<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        req: rpc::get_sequence_and_project_shared_data::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<Output = rpc::get_sequence_and_project_shared_data::Result>
                + Send,
        >,
    > {
        Box::pin(async move {
            let sequence = SequenceDocumentGet {
                pk_id: req.sequence_id,
            }
            .run()
            .await
            .map_err(|error| {
                rpc::get_sequence_and_project_shared_data::Error::Unknown(error.to_string())
            })?;

            let project = ProjectDocumentGet {
                pk_id: sequence.project_id,
            }
            .run()
            .await
            .map_err(|error| {
                rpc::get_sequence_and_project_shared_data::Error::Unknown(error.to_string())
            })?;

            Ok(rpc::get_sequence_and_project_shared_data::Response {
                sequence_json: sequence.sequence_json_string().map_err(|error| {
                    rpc::get_sequence_and_project_shared_data::Error::Unknown(error.to_string())
                })?,
                project_shared_data_json: project.shared_data_json,
            })
        })
    }

    fn delete_sequence<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::delete_sequence::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::delete_sequence::Result> + Send>,
    > {
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::delete_sequence::Error::Unauthorized);
            }
            let session = session.unwrap();

            let sequence = SequenceDocumentGet {
                pk_id: req.sequence_id,
            }
            .run()
            .await
            .map_err(|error| rpc::delete_sequence::Error::Unknown(error.to_string()))?;

            let is_project_editor = crate::services()
                .project_service
                .is_project_editor(session.user_id, sequence.project_id)
                .await
                .map_err(|error| rpc::delete_sequence::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::delete_sequence::Error::Unauthorized);
            }

            crate::dynamo_db()
                .transact()
                .delete_item(SequenceDocumentDelete {
                    pk_id: req.sequence_id,
                })
                .delete_item(ProjectSequenceDocumentDelete {
                    pk_project_id: sequence.project_id,
                    sk_sequence_id: req.sequence_id,
                })
                .send()
                .await
                .map_err(|error| rpc::delete_sequence::Error::Unknown(error.to_string()))?;

            Ok(rpc::delete_sequence::Response {})
        })
    }

    fn rename_sequence<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::rename_sequence::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::rename_sequence::Result> + Send>,
    > {
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::rename_sequence::Error::Unauthorized);
            }
            let session = session.unwrap();

            let sequence = SequenceDocumentGet {
                pk_id: req.sequence_id,
            }
            .run()
            .await
            .map_err(|error| rpc::rename_sequence::Error::Unknown(error.to_string()))?;

            let is_project_editor = crate::services()
                .project_service
                .is_project_editor(session.user_id, sequence.project_id)
                .await
                .map_err(|error| rpc::rename_sequence::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::rename_sequence::Error::Unauthorized);
            }

            SequenceDocumentUpdate {
                pk_id: req.sequence_id,
                update: move |mut sequence: SequenceDocument| async move {
                    sequence.name = req.new_name;
                    Ok(sequence)
                },
            }
            .run()
            .await
            .map_err(|error| match error {
                crate::storage::dynamo_db::UpdateItemError::Canceled(error) => error,
                crate::storage::dynamo_db::UpdateItemError::NotFound
                | crate::storage::dynamo_db::UpdateItemError::SerializationFailed(_)
                | crate::storage::dynamo_db::UpdateItemError::Conflict
                | crate::storage::dynamo_db::UpdateItemError::Unknown(_) => {
                    rpc::rename_sequence::Error::Unknown(error.to_string())
                }
            })?;

            Ok(rpc::rename_sequence::Response {})
        })
    }
}

fn apply_sequence_patch(
    document: &mut SequenceDocument,
    patch: &rpc::json_patch::Patch,
) -> Result<()> {
    let mut sequence_json_value = document.sequence()?;

    rpc::json_patch::patch(&mut sequence_json_value, patch)?;

    document.json_brotli = compress_serde_using_brotli(&sequence_json_value)?;
    document.last_modified = Some(chrono::Utc::now().timestamp_nanos());

    Ok(())
}

fn compress_serde_using_brotli<T>(value: &T) -> Result<Vec<u8>, serde_json::Error>
where
    T: ?Sized + serde::Serialize,
{
    let buffer_size_16_mb = usize::pow(2, 24);
    let mut output = Vec::new();
    {
        let mut brotli_writer =
            brotli::CompressorWriter::new(&mut output, buffer_size_16_mb, 11, 24);
        serde_json::to_writer(&mut brotli_writer, value)?;
    }

    Ok(output)
}
