use super::*;

pub(crate) type CleanUpFnOnce = Option<Box<dyn FnOnce()>>;

pub(crate) fn handle_effect<CleanUp: Into<EffectCleanUpType>>(
    ctx: &RenderCtx,
    title: impl AsRef<str>,
    effect: impl FnOnce() -> CleanUp,
) {
    let _ = title;

    let instance = ctx.instance();
    let effect_used_sigs_list = &mut instance.effect_used_sigs_list;
    let effect_clean_up_list = &mut instance.effect_clean_up_list;

    let effect_index = ctx
        .effect_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let is_first_run = ctx.instance().is_first_render;

    let used_sig_updated = || {
        let used_sigs = effect_used_sigs_list.get(effect_index).unwrap();
        used_sigs
            .iter()
            .any(|used_sig_id| ctx.is_sig_updated(used_sig_id))
    };

    if is_first_run || used_sig_updated() {
        clean_used_sigs();
        call_prev_clean_up(effect_clean_up_list, effect_index);
        let clean_up = effect().into();
        save_clean_up(effect_clean_up_list, effect_index, clean_up);
        let used_sig_ids = take_used_sigs();

        update_or_push(effect_used_sigs_list, effect_index, used_sig_ids);
    }
}

fn call_prev_clean_up(effect_clean_up_list: &mut [CleanUpFnOnce], effect_index: usize) {
    let clean_up = effect_clean_up_list.get_mut(effect_index);
    if let Some(clean_up) = clean_up {
        let clean_up = std::mem::take(clean_up);
        if let Some(clean_up) = clean_up {
            clean_up();
        }
    }
}

fn save_clean_up(
    effect_clean_up_list: &mut Vec<CleanUpFnOnce>,
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
    fn to_fn_once(self) -> CleanUpFnOnce;
}
impl EffectCleanUp for () {
    fn to_fn_once(self) -> CleanUpFnOnce {
        None
    }
}
impl<T: 'static + FnOnce()> EffectCleanUp for T {
    fn to_fn_once(self) -> CleanUpFnOnce {
        Some(Box::new(self))
    }
}

pub enum EffectCleanUpType {
    None,
    FnOnce(Box<dyn FnOnce()>),
}

impl EffectCleanUp for EffectCleanUpType {
    fn to_fn_once(self) -> CleanUpFnOnce {
        match self {
            Self::None => None,
            Self::FnOnce(f) => Some(f),
        }
    }
}

impl From<()> for EffectCleanUpType {
    fn from(_: ()) -> Self {
        Self::None
    }
}
impl<T: 'static + FnOnce()> From<T> for EffectCleanUpType {
    fn from(f: T) -> Self {
        Self::FnOnce(Box::new(f))
    }
}
