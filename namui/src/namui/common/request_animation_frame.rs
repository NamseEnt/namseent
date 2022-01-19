use once_cell::sync::OnceCell;
use std::sync::Mutex;

type Callback = Box<dyn FnOnce()>;
pub struct CallbackContainer(Callback);
unsafe impl Send for CallbackContainer {}
type CallbackQueue = Vec<CallbackContainer>;

static CALLBACK_QUEUE: OnceCell<Mutex<CallbackQueue>> = OnceCell::new();

fn get_queue() -> std::sync::MutexGuard<'static, CallbackQueue> {
    CALLBACK_QUEUE
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap()
}

pub fn request_animation_frame(callback: impl FnOnce() + 'static) {
    get_queue().push(CallbackContainer(Box::new(callback)))
}

pub(crate) fn invoke_and_flush_all_animation_frame_callbacks() {
    let mut queue = get_queue();
    let queue = std::mem::replace(&mut *queue, Vec::new());
    for callback in queue {
        callback.0();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicBool, AtomicI32},
        Arc,
    };

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn request_animation_frame_should_work() {
        request_animation_frame(|| {});
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn invoke_and_flush_all_animation_frame_callbacks_should_invoke() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();
        request_animation_frame(move || {
            called_clone.store(true, std::sync::atomic::Ordering::Relaxed);
        });
        invoke_and_flush_all_animation_frame_callbacks();
        assert!(called.load(std::sync::atomic::Ordering::Relaxed));
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn invoke_and_flush_all_animation_frame_callbacks_should_flush() {
        let call_count = Arc::new(AtomicI32::new(0));
        let call_count_clone = call_count.clone();
        request_animation_frame(move || {
            call_count_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });
        assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 0);
        invoke_and_flush_all_animation_frame_callbacks();
        assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 1);

        invoke_and_flush_all_animation_frame_callbacks();
        assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 1);
    }
}
