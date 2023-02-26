use super::*;

pub(crate) fn post_render(root_rendering_tree: &RenderingTree) {
    MOUSE_SYSTEM
        .rendering_tree
        .lock()
        .unwrap()
        .clone_from(root_rendering_tree);
}
