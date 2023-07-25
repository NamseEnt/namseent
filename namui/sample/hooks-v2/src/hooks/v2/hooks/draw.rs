use super::*;
use crate::hooks::RENDERING_TREE;

pub(crate) fn draw(root_holder: &ComponentHolder) {
    // namui::log!("root_holder: {:#?}", root_holder);
    let rendering_tree = holder_to_rendering_tree(root_holder);
    // namui::log!("rendering_tree: {:#?}", rendering_tree);
    RENDERING_TREE
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .replace(rendering_tree);

    namui::event::send(namui::NamuiEvent::NoUpdateJustRender);
}

fn holder_to_rendering_tree(holder: &ComponentHolder) -> RenderingTree {
    // namui::log!("me: {:#?}", holder);
    let component = holder.component.get().unwrap().as_ref();
    if let Some(rendering_tree) = component.rendering_tree() {
        // namui::log!("i am rendering_tree: {:#?}", rendering_tree);
        rendering_tree
    } else {
        // namui::log!("i am not rendering_tree");
        namui::render(
            holder
                .children
                .iter()
                .map(|child| holder_to_rendering_tree(child)),
        )
    }
}
