use super::*;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

#[derive(Clone)]
pub(crate) struct TreeContext {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    current_component_parent_id: usize,
    /// order: parent -> left -> right
    component_render_queue: VecDeque<ComponentRenderQueueItem>,
    tree_id_map: HashMap<usize, Vec<usize>>,
    fn_rendering_tree_map: RefCell<HashMap<usize, Option<FnRenderingTree>>>,
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
            })),
        }
    }

    pub(crate) fn end_up_one_component_rendering(self, result: ComponentRenderResult) {
        let mut inner = self.inner.lock().unwrap();
        inner.push_children(result.children, result.component_instance.component_id);
        inner.put_tree_id(result.component_instance.component_id);
        inner.save_fn_rendering_tree(
            result.component_instance.component_id,
            result.fn_rendering_tree,
        );

        if let Some(child) = inner.pop_child().as_ref() {
            inner.current_component_parent_id = child.parent_component_id;

            drop(inner);

            mount_visit(child.component.as_ref(), self);
        } else {
            let rendering_tree = inner.combine_rendering_tree(ROOT_COMPONENT_ID);
            crate::hooks::RENDERING_TREE
                .get()
                .unwrap()
                .lock()
                .unwrap()
                .replace(rendering_tree);
            // Combine for showing on monitor
            // Wait for event
            // Check if the event is for this component
            // If it is, invoke the event
            // todo!()
        }
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
            tree_ids.push(component_id);
        } else {
            self.tree_id_map
                .insert(self.current_component_parent_id, vec![component_id]);
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
}

pub(crate) struct ComponentRenderResult {
    pub(crate) children: Vec<Box<dyn Component>>,
    pub(crate) component_instance: Arc<ComponentInstance>,
    pub(crate) fn_rendering_tree: Option<FnRenderingTree>,
}

pub(crate) type FnRenderingTree = Box<dyn FnOnce(Vec<RenderingTree>) -> RenderingTree>;
