mod ctx;
mod event;
mod mount;
mod set_state;

use super::*;
pub use ctx::*;
pub use event::*;
pub use mount::*;
pub use set_state::*;

#[derive(Clone)]
pub(crate) struct ComponentTree {
    pub(crate) component_instance: Arc<ComponentInstance>,
    pub(crate) children: Vec<ComponentTree>,
    pub(crate) rendering_tree: Option<RenderingTree>,
}

impl ComponentTree {
    pub(crate) fn new(component: &dyn Component) -> Self {
        Self {
            component_instance: Arc::new(ComponentInstance::new(component)),
            children: Vec::new(),
            rendering_tree: None,
        }
    }
}

impl Debug for ComponentTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentTree")
            .field("component_instance", &self.component_instance)
            .field("children", unsafe { &self.children.as_ptr().as_ref() })
            .finish()
    }
}
