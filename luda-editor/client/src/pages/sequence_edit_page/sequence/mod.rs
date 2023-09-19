mod history;
mod syncer;

use self::{history::*, syncer::*};
use namui::Uuid;
use rpc::data::{CutUpdateAction, Sequence};

const MAX_EDIT_HISTORY: usize = 8;

#[derive(Debug)]
pub struct SequenceWrapped {
    history: History<MAX_EDIT_HISTORY, Sequence>,
    syncer: Syncer,
}

impl std::ops::Deref for SequenceWrapped {
    type Target = rpc::data::Sequence;

    fn deref(&self) -> &Self::Target {
        self.history.get()
    }
}

impl SequenceWrapped {
    pub fn new(sequence: rpc::data::Sequence) -> Self {
        Self {
            syncer: Syncer::new(sequence.id),
            history: History::new(sequence),
        }
    }

    pub fn update_cut(&mut self, cut_id: Uuid, action: CutUpdateAction) {
        self.history.update(|sequence| {
            let cut = sequence.cuts.iter_mut().find(|c| c.id == cut_id).unwrap();

            action.clone().update(cut);

            self.syncer.send(SyncReq::UpdateCut { cut_id, action });
        });
    }

    /// TODO: Debounce action that can merge into one
    pub(crate) fn update(&mut self, action: rpc::data::SequenceUpdateAction) {
        self.history.update(|sequence| {
            match action.clone() {
                rpc::data::SequenceUpdateAction::InsertCut { cut, after_cut_id } => {
                    let index = match after_cut_id {
                        Some(after_cut_id) => {
                            sequence
                                .cuts
                                .iter()
                                .position(|c| c.id == after_cut_id)
                                .unwrap()
                                + 1
                        }
                        None => sequence.cuts.len(),
                    };
                    sequence.cuts.insert(index, cut);
                }
                rpc::data::SequenceUpdateAction::RenameSequence { name } => {
                    sequence.name = name;
                }
                rpc::data::SequenceUpdateAction::DeleteCut { cut_id } => {
                    if let Some(position) = sequence.cuts.iter().position(|cut| cut.id == cut_id) {
                        sequence.cuts.swap_remove(position);
                    }
                }
                rpc::data::SequenceUpdateAction::MoveCut {
                    cut_id,
                    after_cut_id,
                } => {
                    if after_cut_id == Some(cut_id) {
                        panic!("Cannot move cut after itself");
                    }
                    let moving_cut_position = sequence
                        .cuts
                        .iter()
                        .position(|cut| cut.id == cut_id)
                        .unwrap();
                    let moving_cut = sequence.cuts.remove(moving_cut_position);
                    let insert_position = match after_cut_id {
                        Some(after_cut_id) => {
                            let position = sequence
                                .cuts
                                .iter()
                                .position(|cut| cut.id == after_cut_id)
                                .unwrap();
                            position + 1
                        }
                        None => 0,
                    };

                    sequence.cuts.insert(insert_position, moving_cut);
                }
                rpc::data::SequenceUpdateAction::SplitCutText {
                    cut_id,
                    new_cut_id,
                    split_at,
                } => {
                    let insert_index =
                        sequence.cuts.iter().position(|c| c.id == cut_id).unwrap() + 1;
                    let cut = sequence.cuts.iter_mut().find(|c| c.id == cut_id).unwrap();

                    let (front_line, back_line) = {
                        let line = cut.line.chars().collect::<Vec<_>>();
                        let (front_line, back_line) = line.split_at(split_at);
                        (front_line.iter().collect(), back_line.iter().collect())
                    };

                    cut.line = front_line;

                    let mut new_cut = cut.clone();
                    new_cut.id = new_cut_id;
                    new_cut.line = back_line;

                    sequence.cuts.insert(insert_index, new_cut);
                }
            }

            self.syncer.send(SyncReq::UpdateSequence {
                sequence_id: sequence.id,
                action,
            });
        });
    }

    #[allow(dead_code)]
    pub fn undo(&mut self) {
        if self.history.undo() {
            self.syncer.send(SyncReq::Undo);
        }
    }
    #[allow(dead_code)]
    pub fn redo(&mut self) {
        if self.history.redo() {
            self.syncer.send(SyncReq::Redo);
        }
    }
}
