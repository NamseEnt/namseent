use crate::*;
use elsa::FrozenIndexSet;
use std::collections::HashSet;

struct GlobalState {
    tree_ctx: TreeContext,
    clippings: Vec<Clipping>,
    saved_clippings: Option<Vec<Clipping>>,
    raw_event: Option<RawEvent>,
    /// pre-calculated matrix stack
    matrix_stack: Vec<TransformMatrix>,
    updated_sigs: FrozenIndexSet<SigId>,
}

static mut GLOBAL_STATE: Option<GlobalState> = None;

pub(crate) fn init(tree_ctx: TreeContext) {
    unsafe {
        GLOBAL_STATE = Some(GlobalState {
            tree_ctx,
            clippings: Default::default(),
            saved_clippings: Default::default(),
            raw_event: Default::default(),
            matrix_stack: vec![TransformMatrix::identity()],
            updated_sigs: Default::default(),
        });
    }
}
pub(crate) fn reset() {
    unsafe {
        init(GLOBAL_STATE.take().unwrap().tree_ctx);
    }
}

pub(crate) fn set_raw_event(raw_event: RawEvent) {
    unsafe {
        GLOBAL_STATE.as_mut().unwrap().raw_event = Some(raw_event);
    }
}

#[derive(Default)]
pub(crate) struct GlobalStatePop {
    clipping: bool,
    top: bool,
    transform_matrix_stack: bool,
}

impl Drop for GlobalStatePop {
    fn drop(&mut self) {
        unsafe {
            let global_state = GLOBAL_STATE.as_mut().unwrap();
            if self.clipping {
                global_state.clippings.pop().unwrap();
            }
            if self.top {
                global_state.clippings = global_state.saved_clippings.take().unwrap();
            }
            if self.transform_matrix_stack {
                global_state.matrix_stack.pop().unwrap();
            }
        }
    }
}

pub(crate) fn push_clipping(clipping: Clipping) -> GlobalStatePop {
    unsafe {
        GLOBAL_STATE.as_mut().unwrap().clippings.push(clipping);
    }
    GlobalStatePop {
        clipping: true,
        ..Default::default()
    }
}

pub(crate) fn top() -> GlobalStatePop {
    unsafe {
        let global_state = GLOBAL_STATE.as_mut().unwrap();
        global_state.saved_clippings = Some(std::mem::take(&mut global_state.clippings));
    }
    GlobalStatePop {
        top: true,
        ..Default::default()
    }
}

pub(crate) fn push_transform_matrix(
    mut_matrix: impl FnOnce(&mut TransformMatrix),
) -> GlobalStatePop {
    unsafe {
        let global_state = GLOBAL_STATE.as_mut().unwrap();
        let mut new = *global_state.matrix_stack.last().unwrap();
        mut_matrix(&mut new);
        global_state.matrix_stack.push(new);
    }
    GlobalStatePop {
        transform_matrix_stack: true,
        ..Default::default()
    }
}

pub(crate) fn no_op() -> GlobalStatePop {
    GlobalStatePop {
        ..Default::default()
    }
}

// gets

pub(crate) fn clippings() -> &'static Vec<Clipping> {
    unsafe { &GLOBAL_STATE.as_ref().unwrap().clippings }
}
pub(crate) fn raw_event() -> &'static Option<RawEvent> {
    unsafe { &GLOBAL_STATE.as_ref().unwrap().raw_event }
}
pub(crate) fn tree_ctx() -> &'static TreeContext {
    unsafe { &GLOBAL_STATE.as_ref().unwrap().tree_ctx }
}
pub(crate) fn matrix() -> &'static TransformMatrix {
    unsafe { GLOBAL_STATE.as_ref().unwrap().matrix_stack.last().unwrap() }
}
pub(crate) fn updated_sigs() -> &'static FrozenIndexSet<SigId> {
    unsafe { &GLOBAL_STATE.as_ref().unwrap().updated_sigs }
}
