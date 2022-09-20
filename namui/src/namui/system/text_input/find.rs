use super::*;

pub fn find_text_input_by_id(
    rendering_tree: &RenderingTree,
    id: &str,
) -> Option<TextInputCustomData> {
    let mut return_value: Option<TextInputCustomData> = None;

    rendering_tree.visit_rln(|rendering_tree, _| {
        match rendering_tree {
            RenderingTree::Special(special) => match special {
                render::SpecialRenderingNode::Custom(custom) => {
                    if let Some(custom_data) = custom.data.downcast_ref::<TextInputCustomData>() {
                        if custom_data.id == id {
                            return_value = Some(custom_data.clone());
                            return ControlFlow::Break(());
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        };
        ControlFlow::Continue(())
    });

    return_value
}
