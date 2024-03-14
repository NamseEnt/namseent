use rustc_hash::FxHashSet;
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicUsize,
};

pub struct Tree<Key, Value> {
    nodes: elsa::FrozenIndexMap<usize, Box<Node<Key, Value>>>,
    node_key: AtomicUsize,
}

// NOTE: UnsafeCell(FrozenIndexMap) implies !Sync. this is duct-tape fix.
unsafe impl<Key, Value> Sync for Tree<Key, Value> {}
unsafe impl<Key, Value> Send for Tree<Key, Value> {}

impl<Key, Value> Default for Tree<Key, Value> {
    fn default() -> Self {
        Self {
            nodes: elsa::FrozenIndexMap::new(),
            node_key: AtomicUsize::new(0),
        }
    }
}

impl<Key: Hash + Eq, Value> Tree<Key, Value> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init_root(&mut self, value: Value) {
        assert!(self.nodes.is_empty());

        self.nodes.insert(
            self.node_key
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            Node::new(0, value).into(),
        );
    }
    pub fn get_root(&'static self) -> &'static Node<Key, Value> {
        self.get_node(0)
    }
    pub fn get_node(&self, index: usize) -> &Node<Key, Value> {
        self.nodes.get(&index).unwrap()
    }
    fn get_next_index(&self) -> usize {
        self.node_key
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
    fn insert(&self, index: usize, node: Node<Key, Value>) {
        self.nodes.insert(index, node.into());
    }
    pub fn retain(&mut self, keep: impl Fn(&Value) -> bool + Clone) {
        let tree_nodes = self.nodes.as_mut();

        let mut deleted_node_keys = FxHashSet::default();

        tree_nodes.retain(|node_key, node| {
            if keep(node) {
                return true;
            }

            deleted_node_keys.insert(*node_key);

            false
        });

        if deleted_node_keys.is_empty() {
            return;
        }

        for (_, node) in tree_nodes.iter_mut() {
            if deleted_node_keys.is_empty() {
                break;
            }

            node.children_keys
                .as_mut()
                .retain(|_, child_node_key| deleted_node_keys.remove(&(**child_node_key)));
        }
    }
}

pub struct Node<Key, Value> {
    pub node_key: usize,
    children_keys: elsa::index_map::FrozenIndexMap<
        Key,
        Box<usize>,
        core::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    >,
    value: Value,
}

impl<Key, Value> Deref for Node<Key, Value> {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<Key, Value> DerefMut for Node<Key, Value> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<Key: Hash + Eq, Value> Node<Key, Value> {
    pub(crate) fn new(node_key: usize, value: Value) -> Self {
        Self {
            node_key,
            children_keys: Default::default(),
            value,
        }
    }
    pub fn get_or_create_child_node(
        &self,
        tree: &'static Tree<Key, Value>,
        user_key: Key,
        value: impl FnOnce() -> Value,
    ) -> &'static Self {
        let index = match self.children_keys.get(&user_key) {
            Some(index) => *index,
            None => {
                let node_key = tree.get_next_index();
                let node = Node::new(node_key, value());
                tree.insert(node_key, node);
                self.children_keys.insert(user_key, node_key.into());
                node_key
            }
        };

        tree.get_node(index)
    }
}
