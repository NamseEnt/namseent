mod ctx;
mod event;
mod mount;

use super::*;
pub use ctx::*;
pub use event::*;
pub use mount::*;

pub(crate) struct ComponentTree {
    pub(crate) component_instance: Arc<ComponentInstance>,
    pub(crate) children: Vec<ComponentTree>,
    pub(crate) rendering_tree: Option<RenderingTree>,
}

impl Debug for ComponentTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentTree")
            .field("component_instance", &self.component_instance)
            .field("children", unsafe { &self.children.as_ptr().as_ref() })
            .finish()
    }
}

fn new_component_id() -> usize {
    static COMPONENT_ID: AtomicUsize = AtomicUsize::new(0);
    COMPONENT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
