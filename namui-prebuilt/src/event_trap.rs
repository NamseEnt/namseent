use namui::prelude::*;

pub fn event_trap(content: RenderingTree) -> RenderingTree {
    content.attach_event(move |builder| {
        builder
            .on_mouse_move_in(|event| event.stop_propagation())
            .on_mouse_move_out(|event| event.stop_propagation())
            .on_mouse_down_in(|event| event.stop_propagation())
            .on_mouse_down_out(|event| event.stop_propagation())
            .on_mouse_up_in(|event| event.stop_propagation())
            .on_mouse_up_out(|event| event.stop_propagation())
            .on_wheel(|event| event.stop_propagation());
        // below don't support stop_propagation
        // .on_key_down(|event| event.stop_propagation())
        // .on_key_up(|event| event.stop_propagation())
    })
}

pub fn event_trap_mouse(content: RenderingTree) -> RenderingTree {
    content.attach_event(|builder| {
        builder
            .on_mouse_down_in(|event| {
                event.stop_propagation();
            })
            .on_mouse_move_in(|event| {
                event.stop_propagation();
            })
            .on_mouse_up_in(|event| {
                event.stop_propagation();
            })
            .on_wheel(|event| {
                let xy = event.namui_context.get_rendering_tree_xy(event.target);
                if let Some(xy) = xy {
                    if let Some(bounding_box) = event.target.get_bounding_box() {
                        let bounding_box = bounding_box + xy;
                        if bounding_box.is_xy_inside(system::mouse::position()) {
                            event.stop_propagation();
                        }
                    }
                }
            });
    })
}
