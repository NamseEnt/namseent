mod v2;

use namui::RenderingTree;
use std::sync::{Arc, Mutex, OnceLock};

static RENDERING_TREE: OnceLock<Arc<Mutex<Option<RenderingTree>>>> = OnceLock::new();

pub(crate) struct Hooks {}

impl Hooks {
    pub(crate) fn new() -> Hooks {
        RENDERING_TREE.get_or_init(|| Arc::new(Mutex::new(None)));
        v2::hooks::start(&v2::MyComponent {});

        Hooks {}
    }

    pub(crate) fn render(&self) -> namui::RenderingTree {
        RENDERING_TREE.get().unwrap().lock().unwrap().clone().into()
    }
}
