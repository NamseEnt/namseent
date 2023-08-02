use super::*;

pub(crate) fn handle_memo<'a, T: 'static + Debug + Send + Sync>(
    ctx: &'a RenderCtx,
    memo: impl FnOnce() -> T,
) -> Sig<'a, T> {
    let instance = ctx.instance.as_ref();
    let mut memo_value_list = instance.memo_value_list.lock().unwrap();
    let mut memo_used_sigs_list = instance.memo_used_sigs_list.lock().unwrap();

    let memo_index = ctx
        .memo_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let is_first_run = memo_value_list.len() <= memo_index;

    let used_sig_updated = || {
        let used_sigs = memo_used_sigs_list.get(memo_index).unwrap();
        used_sigs
            .iter()
            .any(|used_sig_id| ctx.is_sig_updated(used_sig_id))
    };

    let sig_id = SigId {
        id_type: SigIdType::Memo,
        index: memo_index,
        component_id: instance.component_id,
    };

    if is_first_run || used_sig_updated() {
        clean_used_sigs();
        let value = Box::new(memo());
        let used_sig_ids = take_used_sigs();

        update_or_push(&mut memo_value_list, memo_index, value);
        update_or_push(&mut memo_used_sigs_list, memo_index, used_sig_ids);

        let is_used_sig_updated = !is_first_run;
        if is_used_sig_updated {
            ctx.add_sig_updated(sig_id);
        }
    }

    let value: &T = memo_value_list[memo_index]
        .as_ref()
        .as_any()
        .downcast_ref()
        .unwrap();

    let value: &T = unsafe { std::mem::transmute(value) };

    Sig::new(value, sig_id)
}
