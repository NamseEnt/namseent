use super::*;

pub fn use_effect(title: &'static str, use_effect: impl FnOnce()) {
    let _ = title;

    let ctx = ctx();

    let instance = ctx.instance.as_ref();
    let mut effect_used_signals_list = instance.effect_used_signals_list.lock().unwrap();

    let effect_index = ctx
        .effect_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let is_first_run = || {
        ctx.instance
            .is_first_render
            .load(std::sync::atomic::Ordering::SeqCst)
    };

    let used_signal_updated = || {
        if let ContextFor::SetState {
            set_state_item: _,
            updated_signals,
            children: _,
        } = &ctx.context_for
        {
            let used_signals = effect_used_signals_list.get(effect_index).unwrap();
            let updated_signals = updated_signals.lock().unwrap();

            used_signals.iter().any(|used_signal_id| {
                updated_signals
                    .iter()
                    .any(|updated_signal_id| updated_signal_id == used_signal_id)
            })
        } else {
            false
        }
    };

    if is_first_run() || used_signal_updated() {
        clean_used_signals();
        use_effect();
        let used_signal_ids = take_used_signals();

        update_or_push(&mut effect_used_signals_list, effect_index, used_signal_ids);
    }
}
