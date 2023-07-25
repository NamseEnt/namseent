use super::*;

pub(crate) fn handle_effect<'a>(ctx: &'a Context, effect: impl FnOnce()) {
    namui::log!("handle_effect");
    let instance = ctx.instance.as_ref();
    let mut effect_used_signals_list = instance.effect_used_signals_list.lock().unwrap();
    let effect_index = ctx
        .effect_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let is_first_run = || effect_used_signals_list.len() <= effect_index;

    let used_signal_updated = || {
        let used_signals = effect_used_signals_list.get(effect_index).unwrap();
        ctx.is_used_signal_updated(used_signals)
    };

    if is_first_run() || ctx.is_set_state_phase() && used_signal_updated() {
        clean_used_signals();
        effect();
        let used_signal_ids = take_used_signals();
        update_or_push(&mut effect_used_signals_list, effect_index, used_signal_ids);
    }
}
