use crate::documents::*;
use rpc::update_sequence::{Error, Request, Response};

pub async fn update_sequence(
    session: Option<SessionDocument>,
    Request {
        sequence_id,
        action,
    }: Request,
) -> rpc::update_sequence::Result {
    let Some(session) = session else {
        return Err(Error::Unauthorized);
    };

    let mut sequence_index_document = SequenceIndexDocumentGetWithVersion { pk_id: sequence_id }
        .run()
        .await
        .map_err(|error| match error {
            crate::storage::dynamo_db::GetItemError::NotFound => Error::SequenceNotFound,
            _ => Error::Unknown(error.to_string()),
        })?;

    let is_project_editor = crate::apis::project::shared::is_project_editor(
        session.user_id,
        sequence_index_document.project_id,
    )
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    if !is_project_editor {
        return Err(Error::Forbidden);
    }

    let mut sequence_document = SequenceDocumentGet {
        pk_id: sequence_id,
        sk_index: sequence_index_document.index,
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    sequence_index_document.index.increase();
    sequence_index_document.undoable_count.increase();
    sequence_index_document.redoable_count.make_zero();
    sequence_document.index.increase();

    let transact = crate::dynamo_db()
        .transact_with_cancel::<Error>()
        .manual_update_item(sequence_index_document);
    match action {
        rpc::data::SequenceUpdateAction::InsertCut { cut, after_cut_id } => {
            let cut_insert_index = match after_cut_id {
                Some(after_cut_id) => sequence_document
                    .cuts
                    .iter()
                    .position(|cut| cut.cut_id == after_cut_id)
                    .ok_or(Error::CutNotFound)?,
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
        rpc::data::SequenceUpdateAction::DeleteCut { cut_id } => {
            let cut_position = sequence_document
                .cuts
                .iter()
                .position(|cut| cut.cut_id == cut_id)
                .ok_or(Error::CutNotFound)?;

            sequence_document.cuts.remove(cut_position);
            transact
        }
        rpc::data::SequenceUpdateAction::MoveCut(move_cut_action) => {
            let cut_id = move_cut_action.cut_id();
            let after_cut_id = move_cut_action.after_cut_id();
            let moving_cut_position = sequence_document
                .cuts
                .iter()
                .position(|cut| cut.cut_id == cut_id)
                .unwrap();
            let moving_cut = sequence_document.cuts.remove(moving_cut_position);
            let insert_position = match after_cut_id {
                Some(after_cut_id) => {
                    let position = sequence_document
                        .cuts
                        .iter()
                        .position(|cut| cut.cut_id == after_cut_id)
                        .unwrap();
                    position + 1
                }
                None => 0,
            };

            sequence_document.cuts.insert(insert_position, moving_cut);
            transact
        }
        rpc::data::SequenceUpdateAction::SplitCutText {
            cut_id,
            new_cut_id,
            split_at,
        } => {
            let cut_insert_index = sequence_document
                .cuts
                .iter()
                .position(|cut| cut.cut_id == cut_id)
                .ok_or(Error::CutNotFound)?;

            let cut = sequence_document
                .cut_mut(cut_id)
                .ok_or(Error::CutNotFound)?;
            let cut_index = cut.index;
            let mut cut_document = SequenceCutDocumentGet {
                pk_sequence_id: sequence_id,
                pk_cut_id: cut_id,
                sk_cut_index: cut_index,
            }
            .run()
            .await
            .map_err(|error| Error::Unknown(error.to_string()))?;

            let (front_line, back_line) = {
                let line = cut_document.cut.line.chars().collect::<Vec<_>>();
                let (front_line, back_line) = line.split_at(split_at);
                (front_line.iter().collect(), back_line.iter().collect())
            };

            cut.index.increase();
            cut_document.cut_index.increase();
            cut_document.cut.line = front_line;

            let new_cut_index = CircularIndex::new();
            let mut new_cut = cut_document.cut.clone();
            new_cut.line = back_line;
            sequence_document.cuts.insert(
                cut_insert_index,
                CutIndex {
                    cut_id: new_cut_id,
                    index: new_cut_index,
                },
            );
            let new_cut_document = SequenceCutDocument {
                sequence_id,
                cut_id: new_cut_id,
                cut_index: new_cut_index,
                cut: new_cut,
            };

            transact
                .put_item(cut_document)
                .create_item(new_cut_document)
        }
    }
    .put_item(sequence_document)
    .send()
    .await
    .map_err(|error| match error {
        crate::storage::dynamo_db::TransactError::Canceled(canceled) => canceled,
        crate::storage::dynamo_db::TransactError::Unknown(unknown) => {
            Error::Unknown(unknown.to_string())
        }
    })?;

    Ok(Response {})
}
