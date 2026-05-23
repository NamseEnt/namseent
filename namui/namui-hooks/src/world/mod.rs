mod public;

use crate::*;
use elsa::*;
use rustc_hash::FxHashSet;
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    sync::{atomic::AtomicBool, mpsc},
};

pub struct World {
    composers: FrozenIndexMap<ComposerId, Box<Composer>>,
    instances: FrozenIndexMap<InstanceId, Box<Instance>>,
    frozen_instances: RefCell<BTreeMap<ChildKeyChain, FrozenInstance>>,
    pub(crate) frozen_atoms: RefCell<Vec<Vec<u8>>>,
    set_state_rx: mpsc::Receiver<SetStateItem>,
    pub(crate) set_state_tx: &'static mpsc::Sender<SetStateItem>,
    updated_sig_ids: FrozenIndexSet<Box<SigId>>,
    get_now: Box<dyn Fn() -> Instant>,
    record_used_sig_ids: RefCell<Vec<SigId>>,
    pub(crate) atom_list: FrozenVec<Box<dyn Value>>,
    atom_index: Cell<usize>,
    pub(crate) raw_event: Option<RawEvent>,
    pub(crate) is_stop_event_propagation: AtomicBool,
    frame: u64,
    rendered_instance_count: Cell<usize>,
    rendered_composer_count: Cell<usize>,
    pub(crate) compose_command_arena: RefCell<Vec<ComposeCommandNode>>,
    rt_vec_pool: RefCell<Vec<Vec<RenderingTree>>>,
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
                    .insert(child_key.clone(), child_composer_id.into());

                (self.composers.insert(
                    child_composer_id,
                    Composer::new(parent_composer.child_key_chain.append(child_key.clone())).into(),
                )) as _
            }
        }
    }

    pub(crate) fn get_or_create_instance(
        &self,
        parent_composer: &Composer,
        child_key: ChildKey,
    ) -> (&Composer, &Instance) {
        match parent_composer.component_child_map.get(&child_key) {
            Some(ids) => {
                let child_instance = self.instances.get(&ids.instance_id).unwrap();
                let child_composer = self.composers.get(&ids.composer_id).unwrap();
                (child_composer, child_instance)
            }
            None => {
                let instance_id = InstanceId::generate();
                let composer_id = ComposerId::generate();
                let child_key_chain = parent_composer.child_key_chain.append(child_key.clone());

                parent_composer.component_child_map.insert(
                    child_key,
                    Box::new(ComponentChildIds {
                        instance_id,
                        composer_id,
                    }),
                );

                let child_instance = self.instances.insert(
                    instance_id,
                    Box::new(Instance::new(
                        instance_id,
                        self.frozen_instances.borrow_mut().remove(&child_key_chain),
                        child_key_chain.clone(),
                    )),
                );
                let child_composer = self
                    .composers
                    .insert(composer_id, Composer::new(child_key_chain).into());

                (child_composer, child_instance)
            }
        }
    }

    fn handle_set_states(&mut self) {
        for set_state_item in self.set_state_rx.try_iter() {
            match set_state_item {
                SetStateItem::Set { sig_id, value } => match sig_id {
                    SigId::State { instance_id, index } => {
                        let instance = self.instances.as_mut().get_mut(&instance_id).unwrap();
                        instance.state_list.get_mut()[index] = value;
                        self.add_sig_updated(sig_id);
                    }
                    SigId::Atom { index } => {
                        self.atom_list.as_mut()[index] = value;
                        self.add_sig_updated(sig_id);
                    }
                    SigId::Memo { .. } => unreachable!(),
                    SigId::TrackEq { .. } => todo!(),
                },
                SetStateItem::Mutate { sig_id, mutate } => match sig_id {
                    SigId::State { instance_id, index } => {
                        let instance = self.instances.as_mut().get_mut(&instance_id).unwrap();
                        let value = instance.state_list.get_mut().get_mut(index).unwrap();
                        mutate(value.as_mut());
                        self.add_sig_updated(sig_id);
                    }
                    SigId::Atom { index } => {
                        let value = self.atom_list.as_mut().get_mut(index).unwrap();
                        mutate(value.as_mut());
                        self.add_sig_updated(sig_id);
                    }
                    SigId::Memo { .. } => unreachable!(),
                    SigId::TrackEq { .. } => todo!(),
                },
                SetStateItem::Mutate2 { sig_ids, mutate } => {
                    let (sig_id1, sig_id2) = sig_ids;
                    assert_ne!(sig_id1, sig_id2);

                    match (sig_id1, sig_id2) {
                        (
                            SigId::State {
                                instance_id: instance_id1,
                                index: index1,
                            },
                            SigId::State {
                                instance_id: instance_id2,
                                index: index2,
                            },
                        ) => {
                            let instance_1 = self.instances.get(&instance_id1).unwrap();
                            let instance_2 = self.instances.get(&instance_id2).unwrap();

                            let state_list1 = unsafe { &mut *instance_1.state_list.get() };
                            let state_list2 = unsafe { &mut *instance_2.state_list.get() };

                            let value1 = state_list1.get_mut(index1).unwrap();
                            let value2 = state_list2.get_mut(index2).unwrap();

                            mutate((value1.as_mut(), value2.as_mut()));

                            self.add_sig_updated(sig_id1);
                            self.add_sig_updated(sig_id2);
                        }
                        _ => todo!(),
                    }
                }
                SetStateItem::Mutate3 { sig_ids, mutate } => {
                    let (sig_id1, sig_id2, sig_id3) = sig_ids;
                    assert_ne!(sig_id1, sig_id2);
                    assert_ne!(sig_id1, sig_id3);
                    assert_ne!(sig_id2, sig_id3);

                    match (sig_id1, sig_id2, sig_id3) {
                        (
                            SigId::State {
                                instance_id: instance_id1,
                                index: index1,
                            },
                            SigId::State {
                                instance_id: instance_id2,
                                index: index2,
                            },
                            SigId::State {
                                instance_id: instance_id3,
                                index: index3,
                            },
                        ) => {
                            let instance_1 = self.instances.get(&instance_id1).unwrap();
                            let instance_2 = self.instances.get(&instance_id2).unwrap();
                            let instance_3 = self.instances.get(&instance_id3).unwrap();

                            let state_list1 = unsafe { &mut *instance_1.state_list.get() };
                            let state_list2 = unsafe { &mut *instance_2.state_list.get() };
                            let state_list3 = unsafe { &mut *instance_3.state_list.get() };

                            let value1 = state_list1.get_mut(index1).unwrap();
                            let value2 = state_list2.get_mut(index2).unwrap();
                            let value3 = state_list3.get_mut(index3).unwrap();

                            mutate((value1.as_mut(), value2.as_mut(), value3.as_mut()));

                            self.add_sig_updated(sig_id1);
                            self.add_sig_updated(sig_id2);
                            self.add_sig_updated(sig_id3);
                        }
                        _ => todo!(),
                    }
                }
                SetStateItem::Mutate4 { sig_ids, mutate } => {
                    let (sig_id1, sig_id2, sig_id3, sig_id4) = sig_ids;
                    assert_ne!(sig_id1, sig_id2);
                    assert_ne!(sig_id1, sig_id3);
                    assert_ne!(sig_id1, sig_id4);
                    assert_ne!(sig_id2, sig_id3);
                    assert_ne!(sig_id2, sig_id4);
                    assert_ne!(sig_id3, sig_id4);

                    match (sig_id1, sig_id2, sig_id3, sig_id4) {
                        (
                            SigId::State {
                                instance_id: instance_id1,
                                index: index1,
                            },
                            SigId::State {
                                instance_id: instance_id2,
                                index: index2,
                            },
                            SigId::State {
                                instance_id: instance_id3,
                                index: index3,
                            },
                            SigId::State {
                                instance_id: instance_id4,
                                index: index4,
                            },
                        ) => {
                            let instance_1 = self.instances.get(&instance_id1).unwrap();
                            let instance_2 = self.instances.get(&instance_id2).unwrap();
                            let instance_3 = self.instances.get(&instance_id3).unwrap();
                            let instance_4 = self.instances.get(&instance_id4).unwrap();

                            let state_list1 = unsafe { &mut *instance_1.state_list.get() };
                            let state_list2 = unsafe { &mut *instance_2.state_list.get() };
                            let state_list3 = unsafe { &mut *instance_3.state_list.get() };
                            let state_list4 = unsafe { &mut *instance_4.state_list.get() };

                            let value1 = state_list1.get_mut(index1).unwrap();
                            let value2 = state_list2.get_mut(index2).unwrap();
                            let value3 = state_list3.get_mut(index3).unwrap();
                            let value4 = state_list4.get_mut(index4).unwrap();

                            mutate((
                                value1.as_mut(),
                                value2.as_mut(),
                                value3.as_mut(),
                                value4.as_mut(),
                            ));

                            self.add_sig_updated(sig_id1);
                            self.add_sig_updated(sig_id2);
                            self.add_sig_updated(sig_id3);
                            self.add_sig_updated(sig_id4);
                        }
                        _ => todo!(),
                    }
                }
            }
        }
    }

    fn remove_unused_guys(&mut self) {
        let frame = self.frame;
        let mut deleted_instance_ids = FxHashSet::default();
        let mut deleted_composer_ids = FxHashSet::default();

        if self.rendered_instance_count.get() != self.instances.as_mut().len() {
            self.instances.as_mut().retain(|instance_id, instance| {
                let rendered = instance.is_rendered_at(frame);
                if !rendered {
                    deleted_instance_ids.insert(*instance_id);
                }
                rendered
            });
        }

        if self.rendered_composer_count.get() != self.composers.as_mut().len() {
            self.composers.as_mut().retain(|composer_id, composer| {
                let rendered = composer.is_rendered_at(frame);
                if !rendered {
                    deleted_composer_ids.insert(*composer_id);
                }
                rendered
            });
        }

        if deleted_instance_ids.is_empty() && deleted_composer_ids.is_empty() {
            return;
        }

        for (_, composer) in self.composers.as_mut() {
            if !deleted_composer_ids.is_empty() {
                composer
                    .compose_id_map
                    .as_mut()
                    .retain(|_, composer_id| !deleted_composer_ids.contains(composer_id.as_ref()));
            }

            if !deleted_instance_ids.is_empty() {
                composer
                    .component_child_map
                    .as_mut()
                    .retain(|_, ids| !deleted_instance_ids.contains(&ids.instance_id));
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

    pub(crate) fn now(&self) -> Instant {
        (self.get_now)()
    }

    pub(crate) fn frame(&self) -> u64 {
        self.frame
    }

    pub(crate) fn count_rendered_instance(&self) {
        self.rendered_instance_count
            .set(self.rendered_instance_count.get() + 1);
    }

    pub(crate) fn count_rendered_composer(&self) {
        self.rendered_composer_count
            .set(self.rendered_composer_count.get() + 1);
    }

    pub(crate) fn next_atom_index(&self) -> usize {
        let index = self.atom_index.get();
        self.atom_index.set(index + 1);
        index
    }

    pub(crate) fn take_rt_vec(&self) -> Vec<RenderingTree> {
        self.rt_vec_pool.borrow_mut().pop().unwrap_or_default()
    }

    pub(crate) fn recycle_rt_vec(&self, mut vec: Vec<RenderingTree>) {
        if vec.capacity() == 0 {
            return;
        }
        let mut pool = self.rt_vec_pool.borrow_mut();
        if pool.len() < 1024 {
            vec.clear();
            pool.push(vec);
        }
    }

    pub(crate) fn push_compose_command(&self, parent: Option<u32>, command: ComposeCommand) -> u32 {
        let mut arena = self.compose_command_arena.borrow_mut();
        let index = arena.len() as u32;
        arena.push(ComposeCommandNode { command, parent });
        index
    }

    pub(crate) fn record_used_sig(&self, id: SigId) {
        self.record_used_sig_ids.borrow_mut().push(id);
    }

    /// Return value is the index, which you can use for `take_record_used_sigs`.
    pub(crate) fn start_record_used_sigs(&self) -> usize {
        self.record_used_sig_ids.borrow().len()
    }

    pub(crate) fn take_record_used_sigs(&self, start_index: usize) -> Vec<SigId> {
        self.record_used_sig_ids.borrow()[start_index..].to_vec()
    }

    pub(crate) fn is_stop_event_propagation(&self) -> bool {
        self.is_stop_event_propagation
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    fn run_impl(
        &mut self,
        root_component: impl Component,
        event: Option<RawEvent>,
    ) -> RenderingTree {
        self.is_stop_event_propagation
            .store(false, std::sync::atomic::Ordering::Relaxed);
        self.frame += 1;
        self.rendered_instance_count.set(0);
        self.rendered_composer_count.set(0);
        self.compose_command_arena.get_mut().clear();
        reset_render_arena();
        let _arena_scope = enter_arena_scope();
        self.reset_updated_sig_ids();
        self.handle_set_states();

        let root_composer = match self.composers.get(&ComposerId::ROOT) {
            Some(composer) => composer,
            None => self
                .composers
                .insert(ComposerId::ROOT, Composer::new(ChildKeyChain::ROOT).into()),
        };

        let root_instance = match self.instances.get(&InstanceId::ROOT) {
            Some(instance) => instance,
            None => self.instances.insert(
                InstanceId::ROOT,
                Box::new(Instance::new(
                    InstanceId::ROOT,
                    self.frozen_instances
                        .borrow_mut()
                        .remove(&ChildKeyChain::ROOT),
                    ChildKeyChain::ROOT,
                )),
            ),
        };

        self.raw_event = event;

        let rendering_tree =
            render_ctx::run(self, root_component, root_composer, root_instance, None);

        self.remove_unused_guys();
        self.record_used_sig_ids.get_mut().clear();

        rendering_tree
    }
}
