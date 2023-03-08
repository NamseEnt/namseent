use super::*;
use crate::*;
use std::sync::{Arc, Mutex};

struct RenderSystem {
    rendering_tree: Arc<Mutex<RenderingTree>>,
}

lazy_static::lazy_static! {
    static ref RENDER_SYSTEM: Arc<RenderSystem> = Arc::new(RenderSystem::new());
}

pub(crate) async fn init() -> InitResult {
    lazy_static::initialize(&RENDER_SYSTEM);
    Ok(())
}

impl RenderSystem {
    fn new() -> Self {
        Self {
            rendering_tree: Arc::new(Mutex::new(RenderingTree::Empty)),
        }
    }
}

pub(crate) fn last_rendering_tree() -> RenderingTree {
    RENDER_SYSTEM.rendering_tree.lock().unwrap().clone()
}

pub(crate) fn post_render(rendering_tree: &RenderingTree) {
    *RENDER_SYSTEM.rendering_tree.lock().unwrap() = rendering_tree.clone();
}
