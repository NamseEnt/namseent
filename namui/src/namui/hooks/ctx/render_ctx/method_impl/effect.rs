use super::*;
use std::sync::MutexGuard;

pub(crate) fn handle_effect<CleanUp: EffectCleanUp>(
    ctx: &RenderCtx,
    title: impl AsRef<str>,
    effect: impl FnOnce() -> CleanUp,
) {
    let _ = title;

    let instance = ctx.instance.as_ref();
    let mut effect_used_sigs_list = instance.effect_used_sigs_list.lock().unwrap();
    let mut effect_clean_up_list = instance.effect_clean_up_list.lock().unwrap();

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
        clean_used_sigs();
        call_prev_clean_up(&mut effect_clean_up_list, effect_index);
        let clean_up = effect();
        save_clean_up(&mut effect_clean_up_list, effect_index, clean_up);
        let used_sig_ids = take_used_sigs();

        update_or_push(&mut effect_used_sigs_list, effect_index, used_sig_ids);
    }
}

fn call_prev_clean_up(
    effect_clean_up_list: &mut MutexGuard<'_, Vec<Option<Box<dyn FnOnce()>>>>,
    effect_index: usize,
) {
    let clean_up = effect_clean_up_list.get_mut(effect_index);
    if let Some(clean_up) = clean_up {
        let clean_up = std::mem::take(clean_up);
        if let Some(clean_up) = clean_up {
            clean_up();
        }
    }
}

fn save_clean_up(
    effect_clean_up_list: &mut MutexGuard<'_, Vec<Option<Box<dyn FnOnce()>>>>,
    effect_index: usize,
    clean_up: impl EffectCleanUp,
) {
    let clean_up = clean_up.to_fn_once();
    match effect_clean_up_list.get_mut(effect_index) {
        Some(prev_clean_up) => {
            *prev_clean_up = clean_up;
        }
        None => {
            effect_clean_up_list.push(clean_up);
        }
    }
}

pub trait EffectCleanUp {
    fn to_fn_once(self) -> Option<Box<dyn FnOnce()>>;
}
impl EffectCleanUp for () {
    fn to_fn_once(self) -> Option<Box<dyn FnOnce()>> {
        None
    }
}
impl<T: 'static + FnOnce()> EffectCleanUp for T {
    fn to_fn_once(self) -> Option<Box<dyn FnOnce()>> {
        Some(Box::new(self))
    }
}
