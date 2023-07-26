use super::*;
use crate::hooks::RENDERING_TREE;

pub(crate) fn draw(root_holder: &ComponentHolder) {
    let rendering_tree = holder_to_rendering_tree(root_holder);
    RENDERING_TREE
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .replace(rendering_tree);

    namui::event::send(namui::NamuiEvent::NoUpdateJustRender);
}

fn holder_to_rendering_tree(holder: &ComponentHolder) -> RenderingTree {
    if let Some(rendering_tree) = &holder.rendering_tree {
        rendering_tree.clone()
    } else {
        namui::render(
            holder
                .children
                .iter()
                .map(|child| holder_to_rendering_tree(child)),
        )
    }
}
