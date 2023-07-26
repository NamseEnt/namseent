use super::*;

pub(crate) fn handle_memo<'a, T: 'static + Debug + Send + Sync>(
    ctx: &'a Context,
    memo: impl FnOnce() -> T,
) -> Signal<T> {
    let instance = ctx.instance.as_ref();
    let mut memo_used_signals_list = instance.memo_used_signals_list.lock().unwrap();
    let memo_index = ctx
        .memo_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let mut memo_value_list = instance.memo_value_list.lock().unwrap();

    let is_first_run = || memo_value_list.len() <= memo_index;

    let used_signal_updated = || {
        let used_signals = memo_used_signals_list.get(memo_index).unwrap();
        ctx.is_used_signal_updated(used_signals)
    };

    let signal_id = SignalId::Memo(MemoSignalId {
        component_id: instance.component_id,
        memo_index,
    });

    if is_first_run() || ctx.is_set_state_phase() && used_signal_updated() {
        clean_used_signals();
        let value = Arc::new(memo());
        let used_signal_ids = take_used_signals();

        update_or_push(&mut memo_value_list, memo_index, value);
        update_or_push(&mut memo_used_signals_list, memo_index, used_signal_ids);

        ctx.push_used_signal_on_this_ctx(signal_id);
    }
    let value = Arc::downcast(memo_value_list[memo_index].clone().as_arc()).unwrap();

    Signal::new(value, signal_id)
}

pub fn use_memo<'a, T: 'static + Debug + Send + Sync>(memo: impl FnOnce() -> T) -> Signal<'a, T> {
    todo!()
}
