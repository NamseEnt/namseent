use once_cell::sync::OnceCell;
use std::{cmp::Reverse, collections::BinaryHeap, sync::Mutex, time::Duration};

use crate::NamuiImpl;

type Callback = Box<dyn FnOnce()>;
unsafe impl Send for TimeoutCallback {}
struct TimeoutCallback {
    callback: Callback,
    call_at: Duration,
}

impl PartialEq for TimeoutCallback {
    fn eq(&self, other: &Self) -> bool {
        self.call_at == other.call_at
    }
}
impl Eq for TimeoutCallback {}

impl PartialOrd for TimeoutCallback {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeoutCallback {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.call_at.cmp(&other.call_at)
    }
}

static TIMEOUT_CALLBACK_HEAP: OnceCell<Mutex<BinaryHeap<Reverse<TimeoutCallback>>>> =
    OnceCell::new();

fn get_heap() -> std::sync::MutexGuard<'static, BinaryHeap<Reverse<TimeoutCallback>>> {
    TIMEOUT_CALLBACK_HEAP
        .get_or_init(|| Mutex::new(BinaryHeap::new()))
        .lock()
        .unwrap()
}

pub fn set_timeout(callback: impl FnOnce() + 'static, after: Duration) {
    let mut heap = get_heap();
    let now = crate::now();
    let call_at = now + after;
    heap.push(Reverse(TimeoutCallback {
        callback: Box::new(callback),
        call_at,
    }));
}

pub(crate) fn pull_timeout(before_time: Duration) -> Option<Callback> {
    let mut heap = get_heap();
    let timeout = heap.peek();
    if timeout.is_none() {
        return None;
    }
    let timeout = timeout.unwrap();
    if timeout.0.call_at > before_time {
        return None;
    }
    heap.pop().map(|timeout| timeout.0.callback)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::{atomic::AtomicBool, Arc},
        time::Duration,
    };

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn pull_timeout_should_be_fail_if_nothing_pushed() {
        let callback = pull_timeout(crate::now() + Duration::from_millis(2));
        assert!(callback.is_none());
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn pull_timeout_should_pop() {
        set_timeout(move || {}, Duration::from_millis(1));
        assert!(pull_timeout(crate::now() + Duration::from_millis(2)).is_some());
        assert!(pull_timeout(crate::now() + Duration::from_millis(2)).is_none());
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn set_timeout_should_be_able_to_call_callback() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();
        set_timeout(
            move || {
                called_clone.store(true, std::sync::atomic::Ordering::Relaxed);
            },
            Duration::from_millis(1),
        );
        let callback = pull_timeout(crate::now() + Duration::from_millis(2)).unwrap();
        callback();
        assert!(called.load(std::sync::atomic::Ordering::Relaxed));
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn pull_timeout_should_sort_callback_by_time() {
        let vec: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![]));

        let vec1 = vec.clone();
        let vec2 = vec.clone();
        let vec3 = vec.clone();

        set_timeout(
            move || {
                vec2.lock().unwrap().push(2);
            },
            Duration::from_millis(2),
        );
        set_timeout(
            move || {
                vec1.lock().unwrap().push(1);
            },
            Duration::from_millis(1),
        );
        set_timeout(
            move || {
                vec3.lock().unwrap().push(3);
            },
            Duration::from_millis(3),
        );

        while let Some(callback) = pull_timeout(crate::now() + Duration::from_millis(4)) {
            callback();
        }

        assert_eq!(vec.lock().unwrap().as_slice(), &[1, 2, 3]);
    }
}
