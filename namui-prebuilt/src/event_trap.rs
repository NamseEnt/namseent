use namui::prelude::*;

pub fn event_trap(content: RenderingTree) -> RenderingTree {
    content.attach_event(move |builder| {
        builder
            .on_mouse_move_in(|event: MouseEvent| event.stop_propagation())
            .on_mouse_move_out(|event: MouseEvent| event.stop_propagation())
            .on_mouse_down_in(|event: MouseEvent| event.stop_propagation())
            .on_mouse_down_out(|event: MouseEvent| event.stop_propagation())
            .on_mouse_up_in(|event: MouseEvent| event.stop_propagation())
            .on_mouse_up_out(|event: MouseEvent| event.stop_propagation())
            .on_wheel(|event: WheelEvent| event.stop_propagation());
        // below don't support stop_propagation
        // .on_key_down(|event: KeyboardEvent| event.stop_propagation())
        // .on_key_up(|event: KeyboardEvent| event.stop_propagation())
    })
}

pub fn event_trap_mouse(content: RenderingTree) -> RenderingTree {
    content.attach_event(|builder| {
        builder
            .on_mouse_down_in(|event: MouseEvent| {
                event.stop_propagation();
            })
            .on_mouse_move_in(|event: MouseEvent| {
                event.stop_propagation();
            })
            .on_mouse_up_in(|event: MouseEvent| {
                event.stop_propagation();
            })
            .on_wheel(|event: WheelEvent| {
                event.stop_propagation();
            });
    })
}
