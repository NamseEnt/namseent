use super::*;

pub fn use_memo<'a, T: 'static + Debug + Send + Sync>(memo: impl FnOnce() -> T) -> Signal<'a, T> {
    let ctx = ctx();

    let instance = ctx.instance.as_ref();
    let mut memo_value_list = instance.memo_value_list.lock().unwrap();
    let mut memo_used_signals_list = instance.memo_used_signals_list.lock().unwrap();

    let memo_index = ctx
        .memo_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let is_first_run = || memo_value_list.len() <= memo_index;

    let used_signal_updated = || {
        if let ContextFor::SetState {
            set_state_item: _,
            updated_signals,
            children: _,
        } = &ctx.context_for
        {
            let used_signals = memo_used_signals_list.get(memo_index).unwrap();
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

    let signal_id = SignalId {
        id_type: SignalIdType::Memo,
        index: memo_index,
        component_id: instance.component_id,
    };

    if is_first_run() || used_signal_updated() {
        clean_used_signals();
        let value = Box::new(memo());
        let used_signal_ids = take_used_signals();

        update_or_push(&mut memo_value_list, memo_index, value);
        update_or_push(&mut memo_used_signals_list, memo_index, used_signal_ids);

        if let ContextFor::SetState {
            updated_signals, ..
        } = &ctx.context_for
        {
            let mut updated_signals = updated_signals.lock().unwrap();
            updated_signals.insert(signal_id);
        }
    }

    let value: &T = memo_value_list[memo_index]
        .as_ref()
        .as_any()
        .downcast_ref()
        .unwrap();

    let value: &T = unsafe { std::mem::transmute(value) };

    Signal::new(value, signal_id)
}
