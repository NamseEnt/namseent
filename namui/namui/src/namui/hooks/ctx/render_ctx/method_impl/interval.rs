use super::*;

pub(crate) fn handle_interval(
    ctx: &RenderCtx,
    title: impl AsRef<str>,
    duration: Duration,
    job: impl FnOnce(Duration),
) {
    let _ = title;

    let instance = ctx.instance();
    let interval_last_call_at_list = &mut instance.interval_last_call_at_list;

    let interval_index = ctx
        .interval_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let last_call_at = interval_last_call_at_list.get(interval_index);
    let is_first_run = last_call_at.is_none();

    let now = crate::time::now();

    let time_to_call = || {
        let last_call_at = interval_last_call_at_list.get(interval_index).unwrap();
        now - last_call_at >= duration
    };

    if is_first_run || time_to_call() {
        let dt = match last_call_at {
            Some(last_call_at) => now - last_call_at,
            None => Duration::from_secs(0),
        };
        job(dt);
        update_or_push(interval_last_call_at_list, interval_index, now);
    }
}
