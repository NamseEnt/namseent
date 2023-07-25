use super::*;

pub(crate) fn handle_render<'a, C: Component>(
    ctx: &'a Context,
    render: impl 'a + FnOnce() -> C,
) -> Option<Box<dyn Component>> {
    namui::log!("handle_render");
    handle_render_internal(ctx, render)
}

pub(crate) fn handle_render_with_event<'a, C: Component, Event: 'static + Send + Sync>(
    ctx: &'a Context,
    render: impl FnOnce(EventContext<Event>) -> C,
) -> Option<Box<dyn Component>> {
    handle_render_internal(ctx, || {
        let event_context = EventContext::new(ctx.instance.component_id);
        render(event_context)
    })
}

fn handle_render_internal<'a, C: Component>(
    ctx: &'a Context,
    render: impl 'a + FnOnce() -> C,
) -> Option<Box<dyn Component>> {
    namui::log!("handle_render_internal");
    let instance = ctx.instance.as_ref();
    let is_first_run = || {
        instance
            .is_first_render
            .swap(false, std::sync::atomic::Ordering::SeqCst)
    };
    let mut render_used_signals = instance.render_used_signals.lock().unwrap();
    namui::log!("render_used_signals: {:?}", render_used_signals);

    let used_signal_updated = || ctx.is_used_signal_updated(render_used_signals.as_ref());

    if is_first_run() || ctx.is_set_state_phase() && used_signal_updated() {
        clean_used_signals();
        let child = render();
        let used_signal_ids = take_used_signals();
        *render_used_signals = used_signal_ids;

        Some(Box::new(child))
    } else {
        None
    }
}
