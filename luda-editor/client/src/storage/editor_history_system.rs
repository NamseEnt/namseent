use super::*;
use crdt::HistorySystem;
use std::sync::{Arc, Mutex};

pub enum Event {
    Mutated { encoded_update: Box<[u8]> },
}

#[derive(Clone)]
pub struct EditorHistorySystem {
    history_system: Arc<Mutex<HistorySystem<super::system_tree::SystemTree>>>,
}
impl EditorHistorySystem {
    pub fn new(history_system: HistorySystem<super::system_tree::SystemTree>) -> Self {
        Self {
            history_system: Arc::new(Mutex::new(history_system)),
        }
    }
    pub fn mutate<F>(&mut self, f: F)
    where
        F: FnOnce(&mut super::system_tree::SystemTree),
    {
        let now = namui::now();
        let encoded_update = self.history_system.lock().unwrap().mutate(f);
        namui::event::send(Event::Mutated { encoded_update });
    }
    pub fn get_state(&self) -> super::system_tree::SystemTree {
        let state = self.history_system.lock().unwrap().get_state();
        state
    }
    pub fn merge(&mut self, encoded: impl AsRef<[u8]>) {
        self.history_system.lock().unwrap().merge(encoded)
    }
    pub fn encode(&self) -> Box<[u8]> {
        self.history_system.lock().unwrap().encode()
    }
    pub fn export_doc(&self) -> crdt::yrs::Doc {
        self.history_system.lock().unwrap().export_doc()
    }
}
impl std::fmt::Debug for EditorHistorySystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EditorHistorySystem: {:?}",
            self.history_system.lock().unwrap().get_state()
        )
    }
}

impl EditorHistorySystem {
    pub fn mutate_sequence(&mut self, sequence_id: &str, mutate: impl FnOnce(&mut Sequence)) {
        self.mutate(|system_tree| {
            let sequence_index = system_tree
                .sequence_list
                .iter()
                .position(|sequence| sequence.id() == sequence_id)
                .unwrap();

            system_tree.sequence_list.update(sequence_index, mutate);
        })
    }
    pub fn mutate_cut(&mut self, sequence_id: &str, cut_id: &str, mutate: impl FnOnce(&mut Cut)) {
        self.mutate_sequence(sequence_id, |sequence| {
            let cut_index = sequence
                .cuts
                .iter()
                .position(|cut| cut.id() == cut_id)
                .unwrap();
            sequence.cuts.update(cut_index, mutate);
        });
    }
    pub fn mutate_image_clip(
        &mut self,
        image_clip_address: &ImageClipAddress,
        mutate: impl FnOnce(&mut ImageClip),
    ) {
        self.mutate_cut(
            &image_clip_address.sequence_id,
            &image_clip_address.cut_id,
            |cut| {
                let clip_index = cut
                    .image_clips
                    .iter()
                    .position(|clip| clip.id() == image_clip_address.image_clip_id)
                    .unwrap();
                cut.image_clips.update(clip_index, mutate)
            },
        );
    }
    pub fn with_sequence(&self, sequence_id: &str, f: impl FnOnce(&Sequence)) {
        let state = self.get_state();
        let sequence = state
            .sequence_list
            .iter()
            .find(|sequence| sequence.id() == sequence_id)
            .unwrap();
        f(sequence);
    }
    pub fn with_cut(&self, sequence_id: &str, cut_id: &str, f: impl FnOnce(&Cut)) {
        self.with_sequence(sequence_id, |sequence| {
            let cut = sequence.cuts.iter().find(|cut| cut.id() == cut_id).unwrap();
            f(cut);
        });
    }
    pub(crate) fn get_image_layer_image_path(
        &self,
        image_clip_address: &ImageClipAddress,
        layer_index: usize,
    ) -> Option<String> {
        let state = self.get_state();
        let sequence = state
            .sequence_list
            .iter()
            .find(|sequence| sequence.id() == image_clip_address.sequence_id)
            .unwrap();
        let cut = sequence
            .cuts
            .iter()
            .find(|cut| cut.id() == image_clip_address.cut_id)
            .unwrap();
        let image_clip = cut
            .image_clips
            .iter()
            .find(|image_clip| image_clip.id() == image_clip_address.image_clip_id)
            .unwrap();
        let image = image_clip.images.iter().nth(layer_index).unwrap();

        image.image_path.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ImageClipAddress {
    pub sequence_id: String,
    pub cut_id: String,
    pub image_clip_id: String,
}
