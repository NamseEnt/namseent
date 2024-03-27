use super::*;

pub(crate) fn handle_track_eq<'a, T: 'static + Debug + Send + Sync + PartialEq + Clone>(
    ctx: &'a RenderCtx,
    target: &T,
) -> Sig<'a, T> {
    let instance = ctx.instance();
    let track_eq_value_list = &mut instance.track_eq_value_list;

    let track_eq_index = ctx
        .track_eq_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let sig_id = SigId {
        id_type: SigIdType::TrackEq,
        index: track_eq_index,
        component_id: instance.component_instance_id,
    };

    let first_track = || track_eq_value_list.len() <= track_eq_index;
    let not_eq = || {
        let value: &T = track_eq_value_list[track_eq_index]
            .as_ref()
            .as_any()
            .downcast_ref()
            .unwrap();

        value != target
    };

    if first_track() || not_eq() {
        update_or_push(
            track_eq_value_list,
            track_eq_index,
            Box::new(target.clone()),
        );

        ctx.add_sig_updated(sig_id);
    }

    let value: &T = track_eq_value_list[track_eq_index]
        .as_ref()
        .as_any()
        .downcast_ref()
        .unwrap();

    let value: &T = unsafe { std::mem::transmute(value) };

    let sig = Sig::new(value, sig_id);

    sig
}
