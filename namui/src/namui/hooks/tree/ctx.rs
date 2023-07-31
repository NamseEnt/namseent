use super::*;
use crate::RenderingTree;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

#[derive(Clone, Debug)]
pub(crate) struct TreeContext {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    current_component_parent_id: usize,
    /// order: parent -> left -> right
    component_render_queue: VecDeque<ComponentRenderQueueItem>,

    tree_id_map: HashMap<usize, VecDeque<usize>>,
    fn_rendering_tree_map: RefCell<HashMap<usize, Option<FnRenderingTree>>>,
    component_instance_map: HashMap<usize, Arc<ComponentInstance>>,
    updated_sigs: HashSet<SigId>,

    last_render_component_instance_map: HashMap<usize, Arc<ComponentInstance>>,
    last_tree_id_map: HashMap<usize, VecDeque<usize>>,
}

impl Debug for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inner")
            .field(
                "current_component_parent_id",
                &self.current_component_parent_id,
            )
            .field("component_render_queue", &self.component_render_queue.len())
            .field("tree_id_map", &self.tree_id_map)
            .field(
                "fn_rendering_tree_map",
                &self.fn_rendering_tree_map.borrow().keys(),
            )
            .field(
                "component_instance_map",
                &self
                    .component_instance_map
                    .iter()
                    .map(|(k, v)| (k, v.component_id))
                    .collect::<HashMap<_, _>>(),
            )
            .finish()
    }
}

#[derive(Debug)]
struct ComponentRenderQueueItem {
    component: Box<dyn Component>,
    parent_component_id: usize,
}

const ROOT_COMPONENT_ID: usize = usize::MAX;

impl TreeContext {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                current_component_parent_id: ROOT_COMPONENT_ID,
                component_render_queue: Default::default(),
                tree_id_map: Default::default(),
                fn_rendering_tree_map: Default::default(),
                component_instance_map: Default::default(),
                last_render_component_instance_map: Default::default(),
                last_tree_id_map: Default::default(),
                updated_sigs: Default::default(),
            })),
        }
    }

    pub(crate) fn before_re_render(&self) {
        self.inner.lock().unwrap().before_re_render();
    }

    pub(crate) fn end_up_one_component_rendering(mut self, result: ComponentRenderResult) {
        let mut inner = self.inner.lock().unwrap();
        inner.store_component_instance(result.component_instance.clone());
        inner.push_children(result.children, result.component_instance.component_id);
        inner.put_tree_id(result.component_instance.component_id);
        inner.save_fn_rendering_tree(
            result.component_instance.component_id,
            result.fn_rendering_tree,
        );
        inner.flush_updated_sigs();

        if let Some(child) = inner.pop_child().as_ref() {
            inner.current_component_parent_id = child.parent_component_id;
            drop(inner);

            mount_visit(child.component.as_ref(), self);
        } else {
            let rendering_tree = inner.combine_rendering_tree(ROOT_COMPONENT_ID);
            drop(inner);

            crate::log!("rendering_tree: {:#?}", rendering_tree);
            crate::draw_rendering_tree(&rendering_tree);

            let mut is_need_to_re_render = false;
            while !is_need_to_re_render {
                crate::handle_web_event(&rendering_tree);

                self.flush_channel(&mut is_need_to_re_render);
            }
        }
    }
    fn flush_channel(&mut self, is_need_to_re_render: &mut bool) {
        let mut inner = self.inner.lock().unwrap();

        // TODO: Remove this loop, just call once. this loop is for event_callback.
        loop {
            let channel_items = channel::drain();
            crate::log!("channel_items.len(): {}", channel_items.len());
            if channel_items.len() == 0 {
                break;
            }
            for channel_item in channel_items {
                match channel_item {
                    Item::SetStateItem(item) => {
                        *is_need_to_re_render = true;

                        match item {
                            SetStateItem::Set { sig_id, value } => {
                                inner.add_updated_sig(sig_id);
                                let component_instance =
                                    inner.get_component_instance(sig_id.component_id);

                                match sig_id.id_type {
                                    SigIdType::State => {
                                        let mut state_list =
                                            component_instance.state_list.lock().unwrap();

                                        state_list[sig_id.index] =
                                            value.lock().unwrap().take().unwrap();
                                    }
                                    SigIdType::Atom => todo!(),
                                    SigIdType::Memo => unreachable!(),
                                    SigIdType::As => unreachable!(),
                                }
                            }
                            SetStateItem::Mutate { sig_id, mutate } => {
                                inner.add_updated_sig(sig_id);
                                let component_instance =
                                    inner.get_component_instance(sig_id.component_id);

                                match sig_id.id_type {
                                    SigIdType::State => {
                                        let mut state_list =
                                            component_instance.state_list.lock().unwrap();

                                        let state =
                                            state_list.get_mut(sig_id.index).unwrap().as_mut();
                                        let mutate = mutate.lock().unwrap().take().unwrap();
                                        mutate(state);
                                    }
                                    SigIdType::Atom => todo!(),
                                    SigIdType::Memo => unreachable!(),
                                    SigIdType::As => unreachable!(),
                                }
                            }
                        }
                    }
                    Item::EventCallback(event_callback) => {
                        todo!("Remove event callback, just use &dyn Fn closure")
                    }
                }
            }
        }
    }

    pub(crate) fn get_last_component_instance(
        &self,
        static_type_id: StaticTypeId,
    ) -> Option<Arc<ComponentInstance>> {
        self.inner
            .lock()
            .unwrap()
            .get_last_component_instance(static_type_id)
    }

    pub(crate) fn is_sig_updated(&self, sig_id: &SigId) -> bool {
        self.inner.lock().unwrap().is_sig_updated(sig_id)
    }

    pub(crate) fn add_sig_updated(&self, sig_id: SigId) {
        self.inner.lock().unwrap().add_updated_sig(sig_id);
    }
}

impl Inner {
    fn push_children(&mut self, mut children: Vec<Box<dyn Component>>, parent_component_id: usize) {
        while let Some(child) = children.pop() {
            self.component_render_queue
                .push_front(ComponentRenderQueueItem {
                    component: child,
                    parent_component_id,
                })
        }
    }

    fn pop_child(&mut self) -> Option<ComponentRenderQueueItem> {
        self.component_render_queue.pop_front()
    }
    fn put_tree_id(&mut self, component_id: usize) {
        if let Some(tree_ids) = self.tree_id_map.get_mut(&self.current_component_parent_id) {
            tree_ids.push_back(component_id);
        } else {
            self.tree_id_map.insert(
                self.current_component_parent_id,
                vec![component_id].into_iter().collect(),
            );
        }
    }
    fn save_fn_rendering_tree(
        &mut self,
        component_id: usize,
        fn_rendering_tree: Option<FnRenderingTree>,
    ) {
        self.fn_rendering_tree_map
            .borrow_mut()
            .insert(component_id, fn_rendering_tree);
    }
    fn combine_rendering_tree(&self, component_id: usize) -> RenderingTree {
        let children = match self.tree_id_map.get(&component_id) {
            Some(children_ids) => children_ids
                .iter()
                .map(|child_id| self.combine_rendering_tree(*child_id))
                .collect::<Vec<_>>(),
            None => vec![],
        };

        if component_id == ROOT_COMPONENT_ID {
            return RenderingTree::Children(children);
        }

        let fn_rendering_tree = self
            .fn_rendering_tree_map
            .borrow_mut()
            .remove(&component_id)
            .unwrap();
        if let Some(fn_rendering_tree) = fn_rendering_tree {
            fn_rendering_tree(children)
        } else {
            RenderingTree::Children(children)
        }
    }
    fn store_component_instance(&mut self, component_instance: Arc<ComponentInstance>) {
        self.component_instance_map
            .insert(component_instance.component_id, component_instance);
    }
    fn get_component_instance(&self, component_id: usize) -> Arc<ComponentInstance> {
        self.component_instance_map
            .get(&component_id)
            .unwrap()
            .clone()
    }
    fn before_re_render(&mut self) {
        self.current_component_parent_id = ROOT_COMPONENT_ID;
        self.last_render_component_instance_map = std::mem::take(&mut self.component_instance_map);
        self.last_tree_id_map = std::mem::take(&mut self.tree_id_map);
        self.fn_rendering_tree_map.borrow_mut().clear();
    }
    fn get_last_component_instance(
        &mut self,
        static_type_id: StaticTypeId,
    ) -> Option<Arc<ComponentInstance>> {
        // TODO: This is not efficient, need to improve
        let Some(children_ids) = self
            .last_tree_id_map
            .get_mut(&self.current_component_parent_id) else {
                return None;
            };

        while let Some(id) = children_ids.pop_front() {
            let last_instance = self.last_render_component_instance_map.remove(&id).unwrap();
            if last_instance.component_type_id == static_type_id {
                return Some(last_instance);
            }
        }

        None
    }
    fn flush_updated_sigs(&mut self) {
        self.updated_sigs.clear();
    }
    fn is_sig_updated(&self, sig_id: &SigId) -> bool {
        self.updated_sigs.contains(sig_id)
    }
    fn add_updated_sig(&mut self, sig_id: SigId) {
        self.updated_sigs.insert(sig_id);
    }
}

pub(crate) struct ComponentRenderResult {
    pub(crate) children: Vec<Box<dyn Component>>,
    pub(crate) component_instance: Arc<ComponentInstance>,
    pub(crate) fn_rendering_tree: Option<FnRenderingTree>,
}

pub(crate) type FnRenderingTree = Box<dyn FnOnce(Vec<RenderingTree>) -> RenderingTree>;
