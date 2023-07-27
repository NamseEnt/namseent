use super::*;

pub fn use_effect(title: &'static str, use_effect: impl FnOnce()) {
    let _ = title;

    let ctx = ctx();

    let instance = ctx.instance.as_ref();
    let mut effect_used_sigs_list = instance.effect_used_sigs_list.lock().unwrap();

    let effect_index = ctx
        .effect_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let is_first_run = || {
        ctx.instance
            .is_first_render
            .load(std::sync::atomic::Ordering::SeqCst)
    };

    let used_sig_updated = || {
        if let ContextFor::SetState {
            set_state_item: _,
            updated_sigs,
            children_tree: _,
        } = &ctx.context_for
        {
            let used_sigs = effect_used_sigs_list.get(effect_index).unwrap();
            let updated_sigs = updated_sigs.lock().unwrap();

            used_sigs.iter().any(|used_sig_id| {
                updated_sigs
                    .iter()
                    .any(|updated_sig_id| updated_sig_id == used_sig_id)
            })
        } else {
            false
        }
    };

    if is_first_run() || used_sig_updated() {
        clean_used_sigs();
        use_effect();
        let used_sig_ids = take_used_sigs();

        update_or_push(&mut effect_used_sigs_list, effect_index, used_sig_ids);
    }
}
