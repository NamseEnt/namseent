use super::*;
use std::{fmt::Debug, rc::Rc};

impl ComponentCtx<'_> {
    pub fn state<State: 'static + Debug + Send + Sync>(
        &self,
        init: impl FnOnce() -> State,
    ) -> (Sig<State, &State>, SetState<State>) {
        let state_list = &self.instance.state_list;

        let state_index = self
            .state_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let sig_id = SigId::State {
            index: state_index,
            instance_id: self.instance.id,
        };

        let no_state = state_list.len() <= state_index;

        let state = if no_state {
            let state = init();
            state_list.push_get(Box::new(state))
        } else {
            state_list.get(state_index).unwrap()
        };
        let state: &State = state.as_any().downcast_ref().unwrap();

        let set_state = SetState::new(sig_id, self.world.get_set_state_tx());

        let sig = Sig::new(state, sig_id, self.world);

        (sig, set_state)
    }
    pub fn memo<T: 'static + Debug + Send + Sync>(
        &self,
        func: impl FnOnce() -> T,
    ) -> Sig<T, Rc<T>> {
        let mut memo_list = self.instance.memo_list.borrow_mut();

        let memo_index = self
            .memo_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let non_initialized = memo_list.len() <= memo_index;

        let used_sig_updated = || {
            memo_list
                .get(memo_index)
                .unwrap()
                .used_sig_ids
                .iter()
                .any(|used_sig_id| self.is_sig_updated(used_sig_id))
        };

        let sig_id = SigId::Memo {
            index: memo_index,
            instance_id: self.instance.id,
        };

        if non_initialized || used_sig_updated() {
            let record_start_index = self.world.start_record_used_sigs();
            let value = func();
            let used_sig_ids = self.world.take_record_used_sigs(record_start_index);

            let new_memo = Memo {
                value: Rc::new(value),
                used_sig_ids,
            };

            match memo_list.get_mut(memo_index) {
                Some(memo) => {
                    *memo = new_memo;
                }
                None => {
                    assert_eq!(memo_list.len(), memo_index);
                    memo_list.push(new_memo);
                }
            }

            self.add_sig_updated(sig_id);
        }

        let memo = memo_list.get(memo_index).unwrap();

        let value: Rc<T> = Rc::downcast(memo.value.clone().into_rc()).unwrap();

        Sig::new(value, sig_id, self.world)
    }

    pub fn track_eq<T: 'static + Debug + Send + Sync + PartialEq + Clone>(
        &self,
        target: &T,
    ) -> Sig<T, Rc<T>> {
        let mut track_eq_list = self.instance.track_eq_list.borrow_mut();

        let track_eq_index = self
            .track_eq_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let sig_id = SigId::TrackEq {
            instance_id: self.instance.id,
            index: track_eq_index,
        };

        let first_track = || track_eq_list.len() <= track_eq_index;
        let not_eq = || {
            let value: &T = track_eq_list[track_eq_index]
                .as_ref()
                .as_any()
                .downcast_ref()
                .unwrap();

            value != target
        };

        if first_track() || not_eq() {
            let rc_value = Rc::new(target.clone());
            match track_eq_list.get_mut(track_eq_index) {
                Some(value) => {
                    *value = rc_value;
                }
                None => {
                    assert_eq!(track_eq_list.len(), track_eq_index);
                    track_eq_list.push(rc_value);
                }
            }

            self.add_sig_updated(sig_id);
        }

        let value = track_eq_list.get(track_eq_index).unwrap();

        let value: Rc<T> = Rc::downcast(value.clone().into_rc()).unwrap();

        let sig = Sig::new(value, sig_id, self.world);

        sig
    }

    pub(crate) fn effect<CleanUp: Into<EffectCleanUp>>(
        &self,
        title: impl AsRef<str>,
        func: impl FnOnce() -> CleanUp,
    ) {
        let _ = title;

        let mut effect_list = self.instance.effect_list.borrow_mut();

        let effect_index = self
            .effect_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let is_first_run = effect_list.len() <= effect_index;

        let used_sig_updated = || {
            effect_list
                .get(effect_index)
                .unwrap()
                .used_sig_ids
                .iter()
                .any(|used_sig_id| self.is_sig_updated(used_sig_id))
        };

        let call_func = || {
            let record_start_index = self.world.start_record_used_sigs();
            let clean_up = func();
            Effect {
                used_sig_ids: self.world.take_record_used_sigs(record_start_index),
                clean_up: clean_up.into(),
            }
        };

        if is_first_run {
            let effect = call_func();
            effect_list.push(effect);
        } else if used_sig_updated() {
            let effect = effect_list.get_mut(effect_index).unwrap();

            effect.clean_up.take().call();

            *effect = call_func();
        }
    }

    pub fn interval(&self, title: impl AsRef<str>, interval: Duration, job: impl FnOnce(Duration)) {
        let _ = title;

        let mut interval_last_call_at_list = self.instance.interval_called_list.borrow_mut();

        let interval_index = self
            .interval_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let is_first_run = interval_last_call_at_list.len() <= interval_index;

        let now = self.world.now();

        let dt = {
            if is_first_run {
                Duration::from_secs(0)
            } else {
                now - interval_last_call_at_list.get(interval_index).unwrap()
            }
        };

        if is_first_run || dt >= interval {
            job(dt);

            match interval_last_call_at_list.get_mut(interval_index) {
                Some(last_call_at) => {
                    *last_call_at = now;
                }
                None => {
                    interval_last_call_at_list.push(now);
                }
            }
        }
    }

    pub fn controlled_memo<T: 'static + Debug + Send + Sync>(
        &self,
        func: impl FnOnce(Option<T>) -> ControlledMemo<T>,
    ) -> Sig<T, Rc<T>> {
        let mut memo_list = self.instance.memo_list.borrow_mut();

        let memo_index = self
            .memo_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let non_initialized = memo_list.len() <= memo_index;

        let used_sig_updated = || {
            memo_list
                .get(memo_index)
                .unwrap()
                .used_sig_ids
                .iter()
                .any(|used_sig_id| self.is_sig_updated(used_sig_id))
        };

        let sig_id = SigId::Memo {
            index: memo_index,
            instance_id: self.instance.id,
        };

        let run_func = |value| {
            let record_start_index = self.world.start_record_used_sigs();
            let result = func(value);
            let used_sig_ids = self.world.take_record_used_sigs(record_start_index);

            let value = match result {
                ControlledMemo::Changed(value) => {
                    self.add_sig_updated(sig_id);
                    value
                }
                ControlledMemo::Unchanged(value) => value,
            };

            (value, used_sig_ids)
        };

        if non_initialized {
            let (value, used_sig_ids) = run_func(None);

            let memo = Memo {
                value: Rc::new(value),
                used_sig_ids,
            };
            memo_list.push(memo);
        } else if used_sig_updated() {
            let is_memo_last_index = memo_index == memo_list.len() - 1;

            // move last element to memo_index
            let memo = memo_list.swap_remove(memo_index);
            let input: Rc<T> = Rc::downcast(memo.value.into_rc_any()).unwrap();
            let input = Rc::into_inner(input).unwrap();

            let (value, used_sig_ids) = run_func(Some(input));

            let mut memo = Memo {
                value: Rc::new(value),
                used_sig_ids,
            };

            if !is_memo_last_index {
                // swap last element and memo. new memo will be in [memo_index].
                std::mem::swap(&mut memo_list[memo_index], &mut memo);
            }
            // and push last element to the end
            memo_list.push(memo);
        }

        let memo = memo_list.get(memo_index).unwrap();

        let value: Rc<T> = Rc::downcast(memo.value.clone().into_rc()).unwrap();

        Sig::new(value, sig_id, self.world)
    }
    pub fn init_atom<State: 'static + Debug + Send + Sync>(
        &self,
        atom: &'static Atom<State>,
        init: impl Fn() -> State,
    ) -> (Sig<State, &State>, SetState<State>) {
        let atom_list = &self.world.atom_list;

        let atom_index = atom.init(self.world.get_set_state_tx());

        let sig_id = SigId::Atom { index: atom_index };

        let state = match atom_list.get(atom_index) {
            Some(atom_value) => atom_value,
            None => {
                let value = init();
                atom_list.push(Box::new(value));
                atom_list.get(atom_index).unwrap()
            }
        };
        let state: &State = state.as_any().downcast_ref().unwrap();

        let set_state = SetState::new(sig_id, self.world.get_set_state_tx());

        let sig = Sig::new(state, sig_id, self.world);

        (sig, set_state)
    }
    pub fn atom<State: 'static + Debug + Send + Sync>(
        &self,
        atom: &'static Atom<State>,
    ) -> (Sig<State, &State>, SetState<State>) {
        let atom_list = &self.world.atom_list;

        let atom_index = atom.get_index();

        let sig_id = SigId::Atom { index: atom_index };

        let state = atom_list.get(atom_index).unwrap();
        let state: &State = state.as_any().downcast_ref().unwrap();

        let set_state = SetState::new(sig_id, self.world.get_set_state_tx());

        let sig = Sig::new(state, sig_id, self.world);

        (sig, set_state)
    }
}
