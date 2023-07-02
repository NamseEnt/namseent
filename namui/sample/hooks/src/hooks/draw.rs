use super::{component_tree::get_component_tree, List};
use crate::hooks::{component_tree::ComponentTree, Button, RENDERING_TREE};
use namui::prelude::*;
use std::ops::Deref;

pub(crate) fn draw() {
    namui::log!("draw!");

    let tree = get_component_tree();

    let rendering_tree = draw_component_tree_node(tree.deref());
    RENDERING_TREE
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .replace(rendering_tree);
}

fn draw_component_tree_node(node: &dyn ComponentTree) -> RenderingTree {
    let mut vec = Vec::new();
    for child in node.children() {
        if let Some(button) = child.component.as_any().downcast_ref::<Button>() {
            let on_click = button.on_click.clone();
            vec.push(namui_prebuilt::button::text_button_fit(
                20.px(),
                &button.text,
                Color::BLACK,
                Color::BLACK,
                1.px(),
                Color::WHITE,
                0.px(),
                [MouseButton::Left],
                on_click,
            ))
        } else if let Some(_list) = child.component.as_any().downcast_ref::<List>() {
            vec.push(render(
                draw_component_tree_node(child)
                    .into_iter()
                    .enumerate()
                    .map(|(index, child)| namui::translate(0.px(), 20.px() * index, child)),
            ));
        } else {
            let rendered_by_child = draw_component_tree_node(child);
            match rendered_by_child {
                RenderingTree::Empty => {}
                _ => vec.push(rendered_by_child),
            }
        }
    }

    if vec.is_empty() {
        RenderingTree::Empty
    } else {
        RenderingTree::Children(vec)
    }
}
