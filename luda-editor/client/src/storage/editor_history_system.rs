use super::*;
use std::sync::{Arc, Mutex};

/// Because Rc is used in ycrd, Send does not work, so Arc and Mutex are used.
pub type HistorySystem = crdt::HistorySystem<crate::storage::SystemTree>;
pub type SendableHistorySystem = Arc<Mutex<HistorySystem>>;

pub struct EditorHistorySystem {
    history_system: SendableHistorySystem,
    on_mutated: Box<dyn Fn(&HistorySystem)>,
}
impl EditorHistorySystem {
    pub fn new(
        history_system: SendableHistorySystem,
        on_mutated: Box<dyn Fn(&HistorySystem)>,
    ) -> Self {
        Self {
            history_system,
            on_mutated,
        }
    }
    pub fn mutate<F>(&mut self, f: F)
    where
        F: FnOnce(&mut super::system_tree::SystemTree),
    {
        let mut history_system = self.history_system.lock().unwrap();
        history_system.mutate(f);
        (self.on_mutated)(&history_system);
    }
    pub fn get_state(&self) -> super::system_tree::SystemTree {
        self.history_system.lock().unwrap().get_state()
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
    pub fn mutate_sequence(&mut self, mutate: impl FnOnce(&mut Sequence)) {
        self.mutate(|system_tree| {
            mutate(&mut system_tree.sequence);
        });
    }
    pub fn mutate_cut(&mut self, cut_id: &str, mutate: impl FnOnce(&mut Cut)) {
        self.mutate_sequence(|sequence| {
            let cut_index = sequence
                .cuts
                .iter()
                .position(|cut| cut.id() == cut_id)
                .unwrap();
            sequence.cuts.update(cut_index, mutate);
        });
    }
    #[allow(dead_code)]
    pub fn mutate_image_clip(
        &mut self,
        image_clip_address: &ImageClipAddress,
        mutate: impl FnOnce(&mut ImageClip),
    ) {
        self.mutate_cut(&image_clip_address.cut_id, |cut| {
            let clip_index = cut
                .image_clips
                .iter()
                .position(|clip| clip.id() == image_clip_address.image_clip_id)
                .unwrap();
            cut.image_clips.update(clip_index, mutate)
        });
    }
    pub fn with_sequence(&self, f: impl FnOnce(&Sequence)) {
        let state = self.get_state();
        f(&state.sequence);
    }
    #[allow(dead_code)]
    pub fn with_cut(&self, cut_id: &str, f: impl FnOnce(&Cut)) {
        self.with_sequence(|sequence| {
            let cut = sequence.cuts.iter().find(|cut| cut.id() == cut_id).unwrap();
            f(cut);
        });
    }
    #[allow(dead_code)]
    pub(crate) fn get_image_layer_image_path(
        &self,
        image_clip_address: &ImageClipAddress,
        layer_index: usize,
    ) -> Option<String> {
        let state = self.get_state();
        let cut = state
            .sequence
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
