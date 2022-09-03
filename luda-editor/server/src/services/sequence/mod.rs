mod documents;

use crate::session::SessionDocument;
use documents::*;
use futures::future::try_join_all;
use rpc::base64;
use yrs::updates::{decoder::Decode, encoder::Encode};

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
            let project_sequence_documents = crate::dynamo_db()
                .query::<ProjectSequenceDocument>(req.project_id)
                .await;
            if let Err(error) = project_sequence_documents {
                return Err(rpc::list_project_sequences::Error::Unknown(
                    error.to_string(),
                ));
            }
            let project_sequence_documents = project_sequence_documents.unwrap();

            let sequence_name_and_ids = try_join_all(project_sequence_documents.into_iter().map(
                |project_sequence_document| async move {
                    match crate::dynamo_db()
                        .get_item::<SequenceDocument>(&project_sequence_document.sequence_id, None)
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
                .is_project_editor(&session.user_id, &req.project_id)
                .await
                .map_err(|error| rpc::create_sequence::Error::Unknown(error.to_string()))?;

            if !is_project_editor {
                return Err(rpc::create_sequence::Error::Unauthorized);
            }

            let sequence_id = nanoid::nanoid!();

            crate::dynamo_db()
                .transact()
                .create_item(SequenceDocument {
                    id: sequence_id.clone(),
                    project_id: req.project_id.clone(),
                    name: req.name,
                    yrs_update_v2_base64: None,
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

    fn update_client_sequence<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        req: rpc::update_client_sequence::Request,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = rpc::update_client_sequence::Result> + Send>,
    > {
        Box::pin(async move {
            let sequence = crate::dynamo_db()
                .get_item::<SequenceDocument>(req.sequence_id, None)
                .await
                .map_err(|error| rpc::update_client_sequence::Error::Unknown(error.to_string()))?;
            if sequence.yrs_update_v2_base64.is_none() {
                return Err(rpc::update_client_sequence::Error::ServerSequenceNotExists);
            }
            if sequence.last_modified == req.e_tag.map(|e_tag| e_tag.parse::<i64>().unwrap()) {
                return Ok(rpc::update_client_sequence::Response::NotModified);
            }
            let yrs_update_v2_base64 = sequence.yrs_update_v2_base64.as_ref().unwrap();

            let client_state_vector = yrs::StateVector::decode_v2(
                &base64::decode(req.client_state_vector_base64).unwrap(),
            )
            .unwrap();
            let yrs_doc = yrs::Doc::with_client_id(0);
            let mut txn = yrs_doc.transact();

            txn.apply_update(
                yrs::Update::decode_v2(&base64::decode(yrs_update_v2_base64).unwrap()).unwrap(),
            );

            let server_state_vector = txn.state_vector().encode_v2();
            let update_for_client = yrs_doc.encode_state_as_update_v2(&client_state_vector);

            Ok(rpc::update_client_sequence::Response::Modified {
                yrs_update_v2_for_client_base64: base64::encode(update_for_client),
                e_tag: sequence.e_tag().unwrap(),
                server_state_vector_base64: base64::encode(&server_state_vector),
            })
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

            let mut update_v2_for_client = None;
            let mut server_state_vector = None;
            let mut e_tag = None;

            crate::dynamo_db()
                .update_item(
                    req.sequence_id.clone(),
                    None,
                    |mut sequence: SequenceDocument| async {
                        let is_project_editor = crate::services()
                            .project_service
                            .is_project_editor(&session.user_id, &sequence.project_id)
                            .await
                            .map_err(|error| {
                                rpc::update_server_sequence::Error::Unknown(error.to_string())
                            })?;

                        if !is_project_editor {
                            return Err(rpc::update_server_sequence::Error::Unauthorized);
                        }

                        let yrs_doc = yrs::Doc::with_client_id(0);
                        let mut txn = yrs_doc.transact();

                        if let Some(yrs_update_v2_base64) = &sequence.yrs_update_v2_base64 {
                            txn.apply_update(
                                yrs::Update::decode_v2(
                                    &base64::decode(yrs_update_v2_base64).unwrap(),
                                )
                                .unwrap(),
                            );
                        }
                        txn.apply_update(
                            yrs::Update::decode_v2(
                                &base64::decode(req.yrs_update_v2_for_server_base64).unwrap(),
                            )
                            .unwrap(),
                        );

                        let client_state_vector = yrs::StateVector::decode_v2(
                            &base64::decode(&req.client_state_vector_base64).unwrap(),
                        )
                        .unwrap();

                        update_v2_for_client = Some(base64::encode(
                            yrs_doc.encode_state_as_update_v2(&client_state_vector),
                        ));
                        server_state_vector = Some(base64::encode(txn.state_vector().encode_v2()));

                        sequence.yrs_update_v2_base64 = Some(base64::encode(
                            yrs_doc.encode_state_as_update_v2(&yrs::StateVector::default()),
                        ));

                        sequence.last_modified = Some(chrono::Utc::now().timestamp_nanos());
                        e_tag = Some(sequence.e_tag().unwrap());
                        Ok(sequence)
                    },
                )
                .await
                .map_err(|error| match error {
                    crate::storage::dynamo_db::UpdateItemError::Canceled(error) => error,
                    crate::storage::dynamo_db::UpdateItemError::NotFound
                    | crate::storage::dynamo_db::UpdateItemError::SerializationFailed(_)
                    | crate::storage::dynamo_db::UpdateItemError::Conflict
                    | crate::storage::dynamo_db::UpdateItemError::Unknown(_) => {
                        rpc::update_server_sequence::Error::Unknown(error.to_string())
                    }
                })?;

            Ok(rpc::update_server_sequence::Response {
                yrs_update_v2_for_client_base64: update_v2_for_client.unwrap(),
                e_tag: e_tag.unwrap(),
                server_state_vector_base64: server_state_vector.unwrap(),
            })
        })
    }
}
