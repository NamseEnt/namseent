mod any_clone_partial_eq;
mod component;
mod component_tree;
mod draw;
mod foo;
mod hooks;
mod native_components;
mod render;
mod update;

use self::{
    component::Component,
    component_tree::put_to_root,
    foo::Foo,
    update::{update_task, UPDATE_REQUEST_TX},
};
pub use hooks::*;
use namui::{spawn_local, RenderingTree};
pub use native_components::*;
use std::sync::{Arc, Mutex, OnceLock};

static RENDERING_TREE: OnceLock<Arc<Mutex<Option<RenderingTree>>>> = OnceLock::new();

pub(crate) struct Hooks {}

impl Hooks {
    pub(crate) fn new() -> Hooks {
        RENDERING_TREE.get_or_init(|| Arc::new(Mutex::new(None)));
        let (update_request_tx, update_request_rx) = tokio::sync::mpsc::unbounded_channel();
        UPDATE_REQUEST_TX.get_or_init(|| update_request_tx);
        spawn_local(update_task(update_request_rx));

        let root = Foo {};

        start(root);

        Hooks {}
    }

    pub(crate) fn render(&self) -> namui::RenderingTree {
        RENDERING_TREE.get().unwrap().lock().unwrap().clone().into()
    }
}

fn start(root_component: impl Component + 'static) {
    put_to_root(root_component);
}
