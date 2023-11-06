use super::super::any_clone_partial_eq::AnyPartialEq;
use std::sync::{atomic::AtomicUsize, Arc, OnceLock};

static mut STORED_DEPS_LIST: OnceLock<Vec<Arc<dyn AnyPartialEq>>> = OnceLock::new();
static STORED_DEPS_LIST_INDEX: AtomicUsize = AtomicUsize::new(0);

pub fn effect<'a>(deps: impl AnyPartialEq + 'static, func: impl FnOnce()) {
    let index = STORED_DEPS_LIST_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    unsafe {
        let stored_deps_list = STORED_DEPS_LIST.get_mut().unwrap();

        if stored_deps_list.len() > index && stored_deps_list.get(index).unwrap().equals(&deps) {
            return;
        }

        if stored_deps_list.len() == index {
            stored_deps_list.push(Arc::new(deps));
        } else {
            stored_deps_list[index] = Arc::new(deps);
        }

        // TODO: Call func after render
        func();
    };
}

pub(crate) fn set_up_effect_deps_list_before_render(deps_list: Vec<Arc<dyn AnyPartialEq>>) {
    unsafe {
        let _ = STORED_DEPS_LIST.set(deps_list);
    }
    STORED_DEPS_LIST_INDEX.store(0, std::sync::atomic::Ordering::SeqCst);
}

pub(crate) fn get_back_effect_deps_list() -> Vec<Arc<dyn AnyPartialEq>> {
    unsafe { STORED_DEPS_LIST.take().unwrap() }
}
