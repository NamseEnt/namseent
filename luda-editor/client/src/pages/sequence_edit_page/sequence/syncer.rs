use namui::{Uuid, spawn_local};
use rpc::data::{CutUpdateAction, SequenceUpdateAction};
use tokio::sync::mpsc::*;

#[derive(Debug, Clone)]
pub enum SyncReq {
    UpdateCut {
        cut_id: Uuid,
        action: CutUpdateAction,
    },
    UpdateSequence {
        sequence_id: Uuid,
        action: SequenceUpdateAction,
    },
    Undo,
    Redo,
}
#[derive(Debug)]
pub struct Syncer {
    tx: UnboundedSender<SyncReq>,
}
impl PartialEq for Syncer {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Syncer {
    pub fn new(sequence_id: Uuid) -> Syncer {
        let (tx, rx) = unbounded_channel();
        run_sync(sequence_id, rx);
        Syncer { tx }
    }

    pub fn send(&self, req: SyncReq) {
        self.tx.send(req).unwrap();
    }
}

fn run_sync(sequence_id: Uuid, mut rx: UnboundedReceiver<SyncReq>) {
    spawn_local(async move {
        while let Some(sync_req) = rx.recv().await {
            loop {
                let result = match sync_req.clone() {
                    SyncReq::UpdateCut { cut_id, action } => crate::RPC
                        .update_sequence_cut(rpc::update_sequence_cut::Request {
                            cut_id,
                            action,
                            sequence_id,
                        })
                        .await
                        .map(|_| ())
                        .map_err(|error| match error {
                            rpc::update_sequence_cut::Error::Unauthorized
                            | rpc::update_sequence_cut::Error::Forbidden
                            | rpc::update_sequence_cut::Error::CutNotFound
                            | rpc::update_sequence_cut::Error::SequenceNotFound => {
                                unreachable!("error: {:?}", error)
                            }
                            rpc::update_sequence_cut::Error::Unknown(error) => {
                                namui::log!("Error update_sequence_cut: {:?}", error);
                            }
                        }),
                    SyncReq::UpdateSequence {
                        sequence_id,
                        action,
                    } => crate::RPC
                        .update_sequence(rpc::update_sequence::Request {
                            action,
                            sequence_id,
                        })
                        .await
                        .map(|_| ())
                        .map_err(|error| match error {
                            rpc::update_sequence::Error::Unauthorized
                            | rpc::update_sequence::Error::Forbidden
                            | rpc::update_sequence::Error::CutNotFound
                            | rpc::update_sequence::Error::SequenceNotFound => {
                                unreachable!("error: {:?}", error)
                            }
                            rpc::update_sequence::Error::Unknown(error) => {
                                namui::log!("Error update_sequence: {:?}", error);
                            }
                        }),
                    SyncReq::Undo => crate::RPC
                        .undo_update(rpc::undo_update::Request { sequence_id })
                        .await
                        .map(|_| ())
                        .map_err(|error| match error {
                            rpc::undo_update::Error::Unauthorized
                            | rpc::undo_update::Error::Forbidden
                            | rpc::undo_update::Error::NotFound
                            | rpc::undo_update::Error::NoMoreUndo => {
                                unreachable!("error: {:?}", error)
                            }
                            rpc::undo_update::Error::Unknown(error) => {
                                namui::log!("Error undo_update: {:?}", error);
                            }
                        }),
                    SyncReq::Redo => crate::RPC
                        .redo_update(rpc::redo_update::Request { sequence_id })
                        .await
                        .map(|_| ())
                        .map_err(|error| match error {
                            rpc::redo_update::Error::Unauthorized
                            | rpc::redo_update::Error::Forbidden
                            | rpc::redo_update::Error::NotFound
                            | rpc::redo_update::Error::NoMoreRedo => {
                                unreachable!("error: {:?}", error)
                            }
                            rpc::redo_update::Error::Unknown(error) => {
                                namui::log!("Error redo_update: {:?}", error);
                            }
                        }),
                };

                match result {
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {
                        namui::time::delay(namui::Time::Sec(1.0)).await;
                    }
                }
            }
        }
    });
}
