use super::{
    any_clone_partial_eq::AnyPartialEq,
    component::Component,
    get_back_effect_deps_list,
    render::Render,
    set_up_atom_before_render, set_up_effect_deps_list_before_render,
    state::{get_back_states, set_up_state_before_render},
    update::{invoke_update, Source},
};
use crate::hooks::component::WireClosures;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex, MutexGuard, OnceLock},
};

static COMPONENT_TREE: OnceLock<Arc<Mutex<ComponentTreeHead>>> = OnceLock::new();
static COMPONENT_KEY_POSITION_MAP: OnceLock<Arc<Mutex<HashMap<Key, TreePosition>>>> =
    OnceLock::new();

#[derive(Debug, Clone)]
pub(crate) struct TreePosition {
    indexes: Vec<usize>,
}
impl TreePosition {
    fn push(self, child_index: usize) -> TreePosition {
        let mut indexes = self.indexes;
        indexes.push(child_index);
        TreePosition { indexes }
    }
}

pub(crate) fn get_component_tree() -> MutexGuard<'static, ComponentTreeHead> {
    COMPONENT_TREE.get().unwrap().lock().unwrap()
}

pub(crate) trait ComponentTree {
    fn children(&self) -> &Vec<ComponentTreeNode>;
    fn children_mut(&mut self) -> &mut Vec<ComponentTreeNode>;
    fn position(&self) -> TreePosition;

    fn get_child(&self, child_index: usize) -> Option<&ComponentTreeNode> {
        self.children().get(child_index)
    }
    fn get_child_mut(&mut self, child_index: usize) -> Option<&mut ComponentTreeNode> {
        self.children_mut().get_mut(child_index)
    }
    fn put_child_component(&mut self, child_index: usize, component: Arc<dyn Component>) -> Key {
        let child_position = self.position().push(child_index);
        let children = self.children_mut();

        let mut position_map = COMPONENT_KEY_POSITION_MAP
            .get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
            .lock()
            .unwrap();

        let child_key = match children.get_mut(child_index) {
            Some(child) => {
                child.component = component;
                child.key()
            }
            None => {
                assert_eq!(child_index, children.len());
                let key = get_next_key();
                children.push(ComponentTreeNode::new(
                    component,
                    key,
                    child_position.clone(),
                ));
                key
            }
        };

        position_map.insert(child_key, child_position);

        child_key
    }
}

#[derive(Debug)]
pub(crate) struct ComponentTreeHead {
    children: Vec<ComponentTreeNode>,
}
impl ComponentTreeHead {
    pub(crate) fn new() -> ComponentTreeHead {
        ComponentTreeHead {
            children: Vec::new(),
        }
    }
}
unsafe impl Send for ComponentTreeHead {}
unsafe impl Sync for ComponentTreeHead {}

impl ComponentTree for ComponentTreeHead {
    fn children(&self) -> &Vec<ComponentTreeNode> {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<ComponentTreeNode> {
        &mut self.children
    }

    fn position(&self) -> TreePosition {
        TreePosition {
            indexes: Vec::new(),
        }
    }
}

pub(crate) struct ComponentTreeNode {
    pub component: Arc<dyn Component>,
    children: Vec<ComponentTreeNode>,
    key: Key,
    position: TreePosition,
    states: Arc<Vec<Arc<dyn AnyPartialEq>>>,
    effect_deps_list: Arc<Vec<Arc<dyn AnyPartialEq>>>,
}
unsafe impl Send for ComponentTreeNode {}
unsafe impl Sync for ComponentTreeNode {}

impl ComponentTree for ComponentTreeNode {
    fn children(&self) -> &Vec<ComponentTreeNode> {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<ComponentTreeNode> {
        &mut self.children
    }

    fn position(&self) -> TreePosition {
        self.position.clone()
    }
}

impl Debug for ComponentTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentTreeNode")
            .field("component", &self.component)
            .field("key", &self.key)
            .field("position", &self.position)
            .field("states", &self.states)
            .field("children", &self.children)
            .finish()
    }
}

impl ComponentTreeNode {
    fn new(component: Arc<dyn Component>, key: Key, position: TreePosition) -> ComponentTreeNode {
        ComponentTreeNode {
            component,
            children: Vec::new(),
            key,
            position,
            states: Arc::new(Vec::new()),
            effect_deps_list: Arc::new(Vec::new()),
        }
    }

    fn key(&self) -> Key {
        self.key
    }

    pub(crate) fn update(&mut self, source: Arc<Source>) {
        let render = self.render_component();

        for (index, child) in render.into_children().enumerate() {
            if let Some(prev_child_node) = self.get_child_mut(index) {
                if !prev_child_node.component.equals(&child) {
                    let prev_child = std::mem::replace(&mut prev_child_node.component, child);
                    let child_node_component = prev_child_node.component.as_ref();

                    WireClosures::wire_closures(prev_child.as_ref(), child_node_component);
                    invoke_update(prev_child_node.key(), source.clone());
                } else {
                }
            } else {
                let child_key = self.put_child_component(index, child);
                invoke_update(child_key, source.clone());
            }
        }
    }

    fn render_component(&mut self) -> Render {
        let states = {
            let states = std::mem::replace(&mut self.states, Arc::new(Vec::new()));
            Arc::into_inner(states).unwrap()
        };
        set_up_state_before_render(self.key, states);

        let effect_deps_list = {
            let effect_deps_list =
                std::mem::replace(&mut self.effect_deps_list, Arc::new(Vec::new()));
            Arc::into_inner(effect_deps_list).unwrap()
        };
        set_up_effect_deps_list_before_render(effect_deps_list);

        set_up_atom_before_render(self.key);

        let render = self.component.render(Render::new());

        let states = get_back_states();
        self.states = Arc::new(states);

        let effect_deps_list = get_back_effect_deps_list();
        self.effect_deps_list = Arc::new(effect_deps_list);

        render
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Key(usize);

pub(crate) fn get_next_key() -> Key {
    static NEXT_KEY: AtomicUsize = AtomicUsize::new(0);
    Key(NEXT_KEY.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
}

pub(crate) fn put_to_root(component: impl Component + 'static) {
    let mut root = COMPONENT_TREE
        .get_or_init(|| Arc::new(Mutex::new(ComponentTreeHead::new())))
        .lock()
        .unwrap();

    let key = root.put_child_component(0, Arc::new(component));

    invoke_update(key, Arc::new(()));
}

pub(crate) fn with_component_tree_of_key_mut(key: Key, func: impl FnOnce(&mut ComponentTreeNode)) {
    let position = {
        let position_map = COMPONENT_KEY_POSITION_MAP
            .get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
            .lock()
            .unwrap();

        position_map.get(&key).unwrap().clone()
    };

    let mut root = COMPONENT_TREE
        .get_or_init(|| Arc::new(Mutex::new(ComponentTreeHead::new())))
        .lock()
        .unwrap();

    let mut node = root.get_child_mut(position.indexes[0]).unwrap();
    for index in position.indexes.iter().skip(1) {
        node = node.get_child_mut(*index).unwrap();
    }

    func(node);
}

#[allow(dead_code)]
pub(crate) fn update_component_state<T: 'static>(
    key: Key,
    state_index: usize,
    next_state_fn: impl FnOnce(&mut T),
) {
    with_component_tree_of_key_mut(key, |node| {
        let states = Arc::make_mut(&mut node.states);
        let inner = Arc::get_mut(states.get_mut(state_index).unwrap()).unwrap();
        next_state_fn(&mut inner.as_any_mut().downcast_mut().unwrap());
    });
}
