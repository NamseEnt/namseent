pub mod documents;

use super::project::documents::*;
use crate::session::SessionDocument;
use documents::*;
use futures::future::try_join_all;

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
            let project_sequence_query = ProjectSequenceDocumentQuery {
                pk_project_id: req.project_id,
                last_sk: None, // TODO
            }
            .run()
            .await
            .map_err(|error| rpc::list_project_sequences::Error::Unknown(error.to_string()))?;

            let sequence_name_and_ids =
                try_join_all(project_sequence_query.documents.into_iter().map(
                    |project_sequence_document| async move {
                        let sequence_index_document = SequenceIndexDocumentGet {
                            pk_id: project_sequence_document.sequence_id,
                        }
                        .run()
                        .await
                        .map_err(|error| {
                            rpc::list_project_sequences::Error::Unknown(error.to_string())
                        })?;

                        let sequence_document = SequenceDocumentGet {
                            pk_id: project_sequence_document.sequence_id,
                            sk_index: sequence_index_document.index,
                        }
                        .run()
                        .await
                        .map_err(|error| {
                            rpc::list_project_sequences::Error::Unknown(error.to_string())
                        })?;

                        Ok(rpc::list_project_sequences::SequenceNameAndId {
                            id: sequence_document.id,
                            name: sequence_document.name,
                        })
                    },
                ))
                .await?;

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

            crate::dynamo_db()
                .transact()
                .create_item(SequenceIndexDocument {
                    id: sequence_id,
                    project_id: req.project_id,
                    index: CircularIndex::new(),
                    undoable_count: 0,
                    redoable_count: 0,
                })
                .create_item(SequenceDocument {
                    id: sequence_id,
                    index: CircularIndex::new(),
                    project_id: req.project_id,
                    name: req.name,
                    cuts: vec![],
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

    fn undo_update<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::undo_update::Request { sequence_id }: rpc::undo_update::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::undo_update::Result> + Send>>
    {
        Box::pin(async move {
            let Some(session) = session else {
                return Err(rpc::undo_update::Error::Unauthorized);
            };

            SequenceIndexDocumentUpdate {
                pk_id: sequence_id,
                update: move |mut document| async move {
                    let is_project_editor = crate::services()
                        .project_service
                        .is_project_editor(session.user_id, document.project_id)
                        .await
                        .map_err(|error| rpc::undo_update::Error::Unknown(error.to_string()))?;

                    if !is_project_editor {
                        return Err(rpc::undo_update::Error::Forbidden);
                    }

                    if document.undoable_count == 0 {
                        return Err(rpc::undo_update::Error::NoMoreUndo);
                    }
                    document.undoable_count -= 1;
                    document.redoable_count += 1;
                    document.index.decrease();

                    Ok(document)
                },
            }
            .run()
            .await
            .map_err(|error| match error {
                crate::storage::dynamo_db::UpdateItemError::NotFound => {
                    rpc::undo_update::Error::NotFound
                }
                _ => rpc::undo_update::Error::Unknown(error.to_string()),
            })?;

            Ok(rpc::undo_update::Response {})
        })
    }

    fn redo_update<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::redo_update::Request { sequence_id }: rpc::redo_update::Request,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = rpc::redo_update::Result> + Send>>
    {
        Box::pin(async move {
            let Some(session) = session else {
                return Err(rpc::redo_update::Error::Unauthorized);
            };

            SequenceIndexDocumentUpdate {
                pk_id: sequence_id,
                update: move |mut document| async move {
                    let is_project_editor = crate::services()
                        .project_service
                        .is_project_editor(session.user_id, document.project_id)
                        .await
                        .map_err(|error| rpc::redo_update::Error::Unknown(error.to_string()))?;

                    if !is_project_editor {
                        return Err(rpc::redo_update::Error::Forbidden);
                    }

                    if document.redoable_count == 0 {
                        return Err(rpc::redo_update::Error::NoMoreRedo);
                    }
                    document.redoable_count -= 1;
                    document.undoable_count += 1;
                    document.index.increase();

                    Ok(document)
                },
            }
            .run()
            .await
            .map_err(|error| match error {
                crate::storage::dynamo_db::UpdateItemError::NotFound => {
                    rpc::redo_update::Error::NotFound
                }
                _ => rpc::redo_update::Error::Unknown(error.to_string()),
            })?;

            Ok(rpc::redo_update::Response {})
        })
    }

    fn update_sequence<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::update_sequence::Request {
            sequence_id,
            action,
        }: rpc::update_sequence::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::update_sequence::Result> + Send>,
    > {
        Box::pin(async move {
            let Some(session) = session else {
                return Err(rpc::update_sequence::Error::Unauthorized);
            };

            let mut sequence_index_document =
                SequenceIndexDocumentGetWithVersion { pk_id: sequence_id }
                    .run()
                    .await
                    .map_err(|error| match error {
                        crate::storage::dynamo_db::GetItemError::NotFound => {
                            rpc::update_sequence::Error::SequenceNotFound
                        }
                        _ => rpc::update_sequence::Error::Unknown(error.to_string()),
                    })?;

            let is_project_editor = crate::services()
                .project_service
                .is_project_editor(session.user_id, sequence_index_document.project_id)
                .await
                .map_err(|error| rpc::update_sequence::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::update_sequence::Error::Forbidden);
            }

            let mut sequence_document = SequenceDocumentGet {
                pk_id: sequence_id,
                sk_index: sequence_index_document.index,
            }
            .run()
            .await
            .map_err(|error| rpc::update_sequence::Error::Unknown(error.to_string()))?;

            sequence_index_document.index.increase();
            sequence_document.index.increase();

            let transact = crate::dynamo_db()
                .transact_with_cancel::<rpc::update_sequence::Error>()
                .manual_update_item(sequence_index_document);
            match action {
                rpc::data::SequenceUpdateAction::InsertCut { cut, after_cut_id } => {
                    let cut_insert_index = match after_cut_id {
                        Some(after_cut_id) => sequence_document
                            .cuts
                            .iter()
                            .position(|cut| cut.cut_id == after_cut_id)
                            .ok_or(rpc::update_sequence::Error::CutNotFound)?,
                        None => sequence_document.cuts.len(),
                    };

                    sequence_document.cuts.insert(
                        cut_insert_index,
                        CutIndex {
                            cut_id: cut.id,
                            index: CircularIndex::new(),
                        },
                    );

                    transact.create_item(SequenceCutDocument {
                        sequence_id,
                        cut_id: cut.id,
                        cut_index: CircularIndex::new(),
                        cut,
                    })
                }
                rpc::data::SequenceUpdateAction::RenameSequence { name } => {
                    sequence_document.name = name;
                    transact
                }
            }
            .put_item(sequence_document)
            .send()
            .await
            .map_err(|error| match error {
                crate::storage::dynamo_db::TransactError::Canceled(canceled) => canceled,
                crate::storage::dynamo_db::TransactError::Unknown(unknown) => {
                    rpc::update_sequence::Error::Unknown(unknown.to_string())
                }
            })?;

            Ok(rpc::update_sequence::Response {})
        })
    }

    fn update_sequence_cut<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::update_sequence_cut::Request {
            sequence_id,
            cut_id,
            action,
        }: rpc::update_sequence_cut::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::update_sequence_cut::Result> + Send>,
    > {
        Box::pin(async move {
            let Some(session) = session else {
                return Err(rpc::update_sequence_cut::Error::Unauthorized);
            };

            let mut sequence_index_document =
                SequenceIndexDocumentGetWithVersion { pk_id: sequence_id }
                    .run()
                    .await
                    .map_err(|error| match error {
                        crate::storage::dynamo_db::GetItemError::NotFound => {
                            rpc::update_sequence_cut::Error::SequenceNotFound
                        }
                        _ => rpc::update_sequence_cut::Error::Unknown(error.to_string()),
                    })?;

            let is_project_editor = crate::services()
                .project_service
                .is_project_editor(session.user_id, sequence_index_document.project_id)
                .await
                .map_err(|error| rpc::update_sequence_cut::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::update_sequence_cut::Error::Forbidden);
            }

            let mut sequence_document = SequenceDocumentGet {
                pk_id: sequence_id,
                sk_index: sequence_index_document.index,
            }
            .run()
            .await
            .map_err(|error| rpc::update_sequence_cut::Error::Unknown(error.to_string()))?;

            sequence_index_document.index.increase();
            sequence_document.index.increase();

            let cut = sequence_document
                .cut_mut(cut_id)
                .ok_or_else(|| rpc::update_sequence_cut::Error::CutNotFound)?;
            let cut_index = cut.index;

            let mut cut_document = SequenceCutDocumentGet {
                pk_sequence_id: sequence_id,
                pk_cut_id: cut_id,
                sk_cut_index: cut_index,
            }
            .run()
            .await
            .map_err(|error| rpc::update_sequence_cut::Error::Unknown(error.to_string()))?;

            cut.index.increase();
            cut_document.cut_index.increase();

            action.update(&mut cut_document.cut);

            crate::dynamo_db()
                .transact_with_cancel::<rpc::update_sequence_cut::Error>()
                .manual_update_item(sequence_index_document)
                .put_item(sequence_document)
                .put_item(cut_document)
                .send()
                .await
                .map_err(|error| match error {
                    crate::storage::dynamo_db::TransactError::Canceled(canceled) => canceled,
                    crate::storage::dynamo_db::TransactError::Unknown(unknown) => {
                        rpc::update_sequence_cut::Error::Unknown(unknown.to_string())
                    }
                })?;

            Ok(rpc::update_sequence_cut::Response {})
        })
    }

    fn get_sequence_and_project_shared_data<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        rpc::get_sequence_and_project_shared_data::Request { sequence_id }
        : rpc::get_sequence_and_project_shared_data::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<Output = rpc::get_sequence_and_project_shared_data::Result>
                + Send,
        >,
    > {
        Box::pin(async move {
            let sequence_index_document = SequenceIndexDocumentGet { pk_id: sequence_id }
                .run()
                .await
                .map_err(|error| match error {
                    crate::storage::dynamo_db::GetItemError::NotFound => {
                        rpc::get_sequence_and_project_shared_data::Error::SequenceNotFound
                    }
                    _ => {
                        rpc::get_sequence_and_project_shared_data::Error::Unknown(error.to_string())
                    }
                })?;

            let sequence_document = SequenceDocumentGet {
                pk_id: sequence_id,
                sk_index: sequence_index_document.index,
            }
            .run()
            .await
            .map_err(|error| {
                rpc::get_sequence_and_project_shared_data::Error::Unknown(error.to_string())
            })?;

            let project = ProjectDocumentGet {
                pk_id: sequence_document.project_id,
            }
            .run()
            .await
            .map_err(|error| {
                rpc::get_sequence_and_project_shared_data::Error::Unknown(error.to_string())
            })?;

            let cut_futures = sequence_document.cuts.iter().map(|cut_index| async move {
                SequenceCutDocumentGet {
                    pk_sequence_id: sequence_id,
                    pk_cut_id: cut_index.cut_id,
                    sk_cut_index: cut_index.index,
                }
                .run()
                .await
                .map_err(|error| {
                    rpc::get_sequence_and_project_shared_data::Error::Unknown(error.to_string())
                })
                .map(|document| document.cut)
            });

            let cuts = futures::future::try_join_all(cut_futures).await?;

            let sequence = rpc::data::Sequence {
                id: sequence_document.id,
                name: sequence_document.name,
                cuts,
            };

            Ok(rpc::get_sequence_and_project_shared_data::Response {
                sequence,
                project_shared_data_json: project.shared_data_json,
            })
        })
    }

    fn delete_sequence<'a>(
        &'a self,
        session: Option<SessionDocument>,
        rpc::delete_sequence::Request { sequence_id }: rpc::delete_sequence::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::delete_sequence::Result> + Send>,
    > {
        Box::pin(async move {
            let Some(session) = session else  {
                return Err(rpc::delete_sequence::Error::Unauthorized);
            };

            // TODO: Remove all using queue

            let sequence_index_document = SequenceIndexDocumentGet { pk_id: sequence_id }
                .run()
                .await
                .map_err(|error| match error {
                    crate::storage::dynamo_db::GetItemError::NotFound => {
                        rpc::delete_sequence::Error::SequenceNotFound
                    }
                    _ => rpc::delete_sequence::Error::Unknown(error.to_string()),
                })?;
            let is_project_editor = crate::services()
                .project_service
                .is_project_editor(session.user_id, sequence_index_document.project_id)
                .await
                .map_err(|error| rpc::delete_sequence::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::delete_sequence::Error::Unauthorized);
            }

            crate::dynamo_db()
                .transact()
                .delete_item(SequenceIndexDocumentDelete { pk_id: sequence_id })
                .delete_item(ProjectSequenceDocumentDelete {
                    pk_project_id: sequence_index_document.project_id,
                    sk_sequence_id: sequence_id,
                })
                .send()
                .await
                .map_err(|error| rpc::delete_sequence::Error::Unknown(error.to_string()))?;

            Ok(rpc::delete_sequence::Response {})
        })
    }
}
