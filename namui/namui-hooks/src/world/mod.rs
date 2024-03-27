mod public;

use crate::*;
use elsa::*;
use rustc_hash::FxHashSet;
use std::sync::{atomic::AtomicBool, mpsc};

pub struct World {
    composers: FrozenIndexMap<ComposerId, Box<Composer>>,
    instances: FrozenIndexMap<InstanceId, Box<Instance>>,
    set_state_tx: mpsc::Sender<SetStateItem>,
    set_state_rx: mpsc::Receiver<SetStateItem>,
    updated_sig_ids: FrozenIndexSet<Box<SigId>>,
    get_now: Box<dyn Fn() -> Instant>,
    record_used_sig_ids: FrozenVec<Box<SigId>>,
    pub(crate) atom_list: FrozenVec<Box<dyn Value>>,
    pub(crate) raw_event: Option<RawEvent>,
    pub(crate) is_stop_event_propagation: AtomicBool,
    pub(crate) sk_calculate: &'static dyn SkCalculate,
}

impl Drop for World {
    fn drop(&mut self) {}
}

impl World {
    pub(crate) fn get_or_create_composer(
        &self,
        parent_composer: &Composer,
        child_key: ChildKey,
    ) -> &Composer {
        match parent_composer.compose_id_map.get(&child_key) {
            Some(child_composer_id) => self.composers.get(child_composer_id).unwrap(),
            None => {
                let child_composer_id = ComposerId::generate();

                parent_composer
                    .compose_id_map
                    .insert(child_key, child_composer_id.into());

                let child_composer = self
                    .composers
                    .insert(child_composer_id, Composer::new().into());

                child_composer
            }
        }
    }

    pub(crate) fn get_or_create_instance(
        &self,
        composer: &Composer,
        child_key: ChildKey,
    ) -> (&Composer, &Instance) {
        let child_instance = self.get_or_create_instance_only_internal(composer, &child_key);
        let child_composer = self.get_or_create_composer(composer, child_key);

        (child_composer, child_instance)
    }

    fn get_or_create_instance_only_internal(
        &self,
        parent_composer: &Composer,
        child_key: &ChildKey,
    ) -> &Instance {
        match parent_composer.instance_id_map.get(child_key) {
            Some(child_instance_id) => self.instances.get(child_instance_id).unwrap(),
            None => {
                let child_instance_id = InstanceId::generate();

                parent_composer
                    .instance_id_map
                    .insert(child_key.clone(), child_instance_id.into());

                let child_instance = self
                    .instances
                    .insert(child_instance_id, Instance::new(child_instance_id).into());

                child_instance
            }
        }
    }

    fn handle_set_states(&mut self) {
        for set_state_item in self.set_state_rx.try_iter() {
            match set_state_item {
                SetStateItem::Set { sig_id, value } => match sig_id {
                    SigId::State { instance_id, index } => {
                        let instance = self.instances.as_mut().get_mut(&instance_id).unwrap();
                        instance.state_list.as_mut()[index] = value;
                        self.add_sig_updated(sig_id);
                    }
                    SigId::Memo { .. } => unreachable!(),
                    SigId::Atom { index } => {
                        self.atom_list.as_mut()[index] = value;
                        self.add_sig_updated(sig_id);
                    }
                },
                SetStateItem::Mutate { sig_id, mutate } => match sig_id {
                    SigId::State { instance_id, index } => {
                        let instance = self.instances.as_mut().get_mut(&instance_id).unwrap();
                        let value = instance.state_list.as_mut().get_mut(index).unwrap();
                        mutate(value.as_mut());
                        self.add_sig_updated(sig_id);
                    }
                    SigId::Memo { .. } => unreachable!(),
                    SigId::Atom { index } => {
                        let value = self.atom_list.as_mut().get_mut(index).unwrap();
                        mutate(value.as_mut());
                        self.add_sig_updated(sig_id);
                    }
                },
            }
        }
    }

    fn remove_unused_guys(&mut self) {
        let mut deleted_instance_ids = FxHashSet::default();

        self.instances.as_mut().retain(|instance_id, instance| {
            let rendered_flag = instance.take_rendered_flag();
            if !rendered_flag {
                deleted_instance_ids.insert(*instance_id);
            }
            rendered_flag
        });

        let mut deleted_composer_ids = FxHashSet::default();

        self.composers.as_mut().retain(|composer_id, composer| {
            let rendered_flag = composer.take_rendered_flag();
            if !rendered_flag {
                deleted_composer_ids.insert(*composer_id);
            }
            rendered_flag
        });

        if deleted_instance_ids.is_empty() && deleted_composer_ids.is_empty() {
            return;
        }

        for (_, composer) in self.composers.as_mut() {
            if deleted_instance_ids.is_empty() && deleted_composer_ids.is_empty() {
                return;
            }

            if !deleted_instance_ids.is_empty() {
                composer
                    .instance_id_map
                    .as_mut()
                    .retain(|_, instance_id| !deleted_instance_ids.remove(instance_id.as_ref()));
            }

            if !deleted_composer_ids.is_empty() {
                composer
                    .compose_id_map
                    .as_mut()
                    .retain(|_, composer_id| !deleted_composer_ids.remove(composer_id.as_ref()));
            }
        }
    }

    pub(crate) fn is_sig_updated(&self, target_sig_id: &SigId) -> bool {
        self.updated_sig_ids.get(target_sig_id).is_some()
    }

    pub(crate) fn add_sig_updated(&self, sig_id: SigId) {
        self.updated_sig_ids.insert(Box::new(sig_id));
    }

    fn reset_updated_sig_ids(&mut self) {
        self.updated_sig_ids.as_mut().clear();
    }

    pub(crate) fn get_set_state_tx(&self) -> &mpsc::Sender<SetStateItem> {
        &self.set_state_tx
    }

    pub(crate) fn now(&self) -> Instant {
        (self.get_now)()
    }

    pub(crate) fn record_used_sig(&self, id: SigId) {
        self.record_used_sig_ids.push(id.into());
    }

    /// Return value is the index, which you can use for `take_record_used_sigs`.
    pub(crate) fn start_record_used_sigs(&self) -> usize {
        self.record_used_sig_ids.len()
    }

    pub(crate) fn take_record_used_sigs(&self, start_index: usize) -> Vec<SigId> {
        let len = self.record_used_sig_ids.len();
        let mut vec = vec![];

        for index in start_index..len {
            vec.push(*self.record_used_sig_ids.get(index).unwrap());
        }
        vec
    }

    pub(crate) fn is_stop_event_propagation(&self) -> bool {
        self.is_stop_event_propagation
            .load(std::sync::atomic::Ordering::Relaxed)
    }
}
