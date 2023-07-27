use super::*;
use crate::hooks::RENDERING_TREE;

pub(crate) fn draw(root_tree: &ComponentTree) {
    let rendering_tree = tree_to_rendering_tree(root_tree);
    RENDERING_TREE
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .replace(rendering_tree);

    namui::event::send(namui::NamuiEvent::NoUpdateJustRender);
}

fn tree_to_rendering_tree(tree: &ComponentTree) -> RenderingTree {
    let children = tree
        .children
        .iter()
        .map(|child| tree_to_rendering_tree(child))
        .collect();
    if let Some(rendering_tree) = &tree.fn_rendering_tree {
        rendering_tree(children)
    } else {
        RenderingTree::Children(children)
    }
}
