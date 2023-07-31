use super::*;

pub fn handle_use_effect(ctx: &RenderCtx, title: &'static str, use_effect: impl FnOnce()) {
    let _ = title;

    let instance = ctx.instance.as_ref();
    let mut effect_used_sigs_list = instance.effect_used_sigs_list.lock().unwrap();

    let effect_index = ctx
        .effect_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let is_first_run = ctx
        .instance
        .is_first_render
        .load(std::sync::atomic::Ordering::SeqCst);

    let used_sig_updated = || {
        let used_sigs = effect_used_sigs_list.get(effect_index).unwrap();
        used_sigs
            .iter()
            .any(|used_sig_id| ctx.is_sig_updated(used_sig_id))
    };

    if is_first_run || used_sig_updated() {
        crate::log!("effect: {title}, is_first_run: {is_first_run}");
        clean_used_sigs();
        use_effect();
        let used_sig_ids = take_used_sigs();

        crate::log!("effect: {title}, used_sig_ids: {:?}", used_sig_ids);

        update_or_push(&mut effect_used_sigs_list, effect_index, used_sig_ids);
    }
}
