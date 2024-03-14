use super::*;

pub(crate) struct HookTree {
    inner: static_tree::Tree<Key, HookInstance>,
}

pub(crate) struct HookNodeWrapper {
    inner: &'static static_tree::Node<Key, HookInstance>,
}

impl HookNodeWrapper {
    pub fn get_or_create_child_node(
        &self,
        user_key: Key,
        value: impl FnOnce() -> HookInstance,
    ) -> Self {
        let child = self
            .inner
            .get_or_create_child_node(hook_tree(), user_key, value);

        HookNodeWrapper { inner: child }
    }

    pub(crate) fn before_render(&mut self) {
        self.inner.before_render();
    }

    pub(crate) fn get_component_instance(&self) -> &'static mut ComponentInstance {
        let HookType::Component { instance } =
            &mut hook_tree().get_node(self.inner.node_key).hook_type
        else {
            unreachable!()
        };
        instance
    }
}

#[derive(Debug)]
pub(crate) enum HookType {
    Component { instance: Box<ComponentInstance> },
    Compose,
}

pub(crate) struct HookInstance {
    pub(crate) hook_type: HookType,
    is_rendered_on_this_tick: bool,
    next_internal_children_id: usize,
}

impl HookInstance {
    pub(crate) fn new(hook_type: HookType) -> Self {
        Self {
            hook_type,
            is_rendered_on_this_tick: Default::default(),
            next_internal_children_id: Default::default(),
        }
    }

    pub(crate) fn before_render(&mut self) {
        self.is_rendered_on_this_tick = false;
        self.next_internal_children_id = 0;
    }
}

impl HookTree {
    pub(crate) fn new() -> Self {
        let mut hook_tree = HookTree {
            inner: static_tree::Tree::new(),
        };
        hook_tree
            .inner
            .init_root(HookInstance::new(HookType::Component {
                instance: ComponentInstance::new().into(),
            }));

        hook_tree
    }
}

pub(crate) fn get_root_node() -> HookNodeWrapper {
    HookNodeWrapper {
        inner: hook_tree().get_root(),
    }
}

pub(crate) fn clear_unrendered() {
    hook_tree_mut().retain(|instance| instance.is_rendered_on_this_tick);
}
