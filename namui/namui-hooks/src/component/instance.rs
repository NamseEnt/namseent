use crate::*;
use std::{
    cell::{RefCell, UnsafeCell},
    sync::atomic::AtomicBool,
};

/// the state of component.
pub(crate) struct Instance {
    pub(crate) id: usize,
    rendered_flag: AtomicBool,
    pub(crate) state_list: UnsafeCell<Vec<Box<dyn Value>>>,
    pub(crate) memo_list: UnsafeCell<Vec<Memo>>,
    pub(crate) track_eq_list: UnsafeCell<Vec<Box<dyn Value>>>,
    pub(crate) track_eq_tuple_list: RefCell<Vec<()>>,
    pub(crate) effect_list: RefCell<Vec<Effect>>,
    pub(crate) interval_called_list: RefCell<Vec<Instant>>,
    pub(crate) abort_handle_list: RefCell<Vec<tokio::task::AbortHandle>>,
}
impl Instance {
    pub(crate) fn new(id: usize) -> Self {
        Self {
            id,
            rendered_flag: Default::default(),
            state_list: Default::default(),
            memo_list: Default::default(),
            track_eq_list: Default::default(),
            track_eq_tuple_list: Default::default(),
            effect_list: Default::default(),
            interval_called_list: Default::default(),
            abort_handle_list: Default::default(),
        }
    }

    pub(crate) fn set_rendered_flag(&self) {
        self.rendered_flag
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub(crate) fn take_rendered_flag(&mut self) -> bool {
        self.rendered_flag
            .swap(false, std::sync::atomic::Ordering::Relaxed)
    }

    pub(crate) fn freeze(mut self) -> Vec<u8> {
        use bytes::BufMut;
        let mut bytes = Vec::new();
        bytes.put_slice(&self.id.to_le_bytes());

        let state_list = std::mem::take(&mut self.state_list).into_inner();
        bytes.put_u16(state_list.len() as u16);
        for state in state_list {
            bytes.put_slice(&state.serialize());
        }

        let memo_list = std::mem::take(&mut self.memo_list).into_inner();
        bytes.put_u16(memo_list.len() as u16);
        for memo in memo_list {
            bytes.put_slice(&memo.serialize());
        }

        let track_eq_list = std::mem::take(&mut self.track_eq_list).into_inner();
        bytes.put_u16(track_eq_list.len() as u16);
        for track_eq in track_eq_list {
            bytes.put_slice(&track_eq.serialize());
        }

        let track_eq_tuple_list = std::mem::take(&mut self.track_eq_tuple_list).into_inner();
        bytes.put_u16(track_eq_tuple_list.len() as u16);
        for track_eq_tuple in track_eq_tuple_list {
            bytes.put_slice(&track_eq_tuple.serialize());
        }

        bytes
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        let effect_list = self.effect_list.take();
        for effect in effect_list {
            effect.clean_up.call();
        }
        let abort_handle_list = self.abort_handle_list.take();
        for abort_handle in abort_handle_list {
            abort_handle.abort();
        }
    }
}

pub(crate) struct Memo {
    pub(crate) value: Box<dyn Value>,
    pub(crate) used_sig_ids: Vec<SigId>,
}

#[derive(Default)]
pub(crate) struct Effect {
    pub(crate) used_sig_ids: Vec<SigId>,
    pub(crate) clean_up: EffectCleanUp,
}
