use super::*;

pub(crate) fn handle_mut_state<State: Send + Sync + Debug + 'static>(
    ctx: &RenderCtx,
    init: impl FnOnce() -> State,
) -> MutState<'_, State> {
    let instance = ctx.instance();
    let mut_state_list = &mut instance.mut_state_list;

    let mut_state_index = ctx
        .mut_state_index
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let sig_id = SigId {
        id_type: SigIdType::MutState,
        index: mut_state_index,
        component_id: instance.component_instance_id,
    };

    let no_state = mut_state_list.len() <= mut_state_index;

    if no_state {
        let state = init();

        update_or_push(mut_state_list, mut_state_index, Box::new(state));
    }

    let state: &mut State = mut_state_list[mut_state_index]
        .as_mut()
        .as_any_mut()
        .downcast_mut()
        .unwrap();

    let state: &mut State = unsafe { std::mem::transmute(state) };

    MutState {
        value: state,
        sig_id,
    }
}

pub struct MutState<'a, T> {
    value: &'a mut T,
    sig_id: SigId,
}

impl<'a, T> AsRef<T> for MutState<'a, T> {
    fn as_ref(&self) -> &T {
        self.value
    }
}

// impl<'a, T> AsMut<T> for MutState<'a, T> {
//     fn as_mut(&mut self) -> &mut T {
//         channel::send(Item::MutStateCalled {
//             sig_id: self.sig_id,
//         });

//         self.value
//     }
// }

impl<'a, T> std::ops::Deref for MutState<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

// impl<'a, T> std::ops::DerefMut for MutState<'a, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         channel::send(Item::MutStateCalled {
//             sig_id: self.sig_id,
//         });

//         self.value
//     }
// }

impl<'a, T> MutState<'a, T> {
    pub fn mutate(&mut self, mutate: impl FnOnce(&mut T)) {
        mutate(self.value);
        channel::send(Item::MutStateCalled {
            sig_id: self.sig_id,
        });
    }
}
