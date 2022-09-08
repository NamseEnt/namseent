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
                        if custom_data.text_input.id == id {
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
pub fn find_front_text_input_on_mouse(
    rendering_tree: &RenderingTree,
    raw_mouse_event: &RawMouseEvent,
) -> Option<TextInputCustomData> {
    let mut return_value: Option<TextInputCustomData> = None;

    rendering_tree.visit_rln(|rendering_tree, utils| {
        match rendering_tree {
            RenderingTree::Special(special) => match special {
                render::SpecialRenderingNode::Custom(custom) => {
                    if let Some(custom_data) = custom.data.downcast_ref::<TextInputCustomData>() {
                        let is_custom_in_mouse = utils.is_xy_in(raw_mouse_event.xy);

                        if is_custom_in_mouse {
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
