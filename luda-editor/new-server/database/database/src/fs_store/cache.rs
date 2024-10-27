use super::*;
use arc_swap::ArcSwap;
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

pub struct Cache {
    inner: ArcSwap<HashMap<Key, Value>>,
    total_max_byte_len: usize,
    stored_byte_len: AtomicU64,
    start_time: Instant,
}
impl Cache {
    pub fn new(total_max_byte_len: usize) -> Self {
        Self {
            inner: Default::default(),
            total_max_byte_len,
            stored_byte_len: Default::default(),
            start_time: Instant::now(),
        }
    }
    pub(crate) fn get(&self, key: Key) -> Option<Option<bytes::Bytes>> {
        self.inner.load().get(&key).map(|value| {
            self.update_last_accessed(value);
            value.bytes.clone()
        })
    }

    pub(crate) fn push(&self, key_values: impl IntoIterator<Item = (Key, Option<Bytes>)>) {
        let mut inner = self.inner.load().as_ref().clone();

        let last_accessed = self.elapsed_u64();
        for (key, bytes) in key_values {
            let value = Value {
                bytes,
                last_accessed: AtomicU64::new(last_accessed),
            };

            self.stored_byte_len
                .fetch_add(value.byte_len() as u64, Ordering::Relaxed);

            if let Some(prev) = inner.insert(key, value) {
                self.stored_byte_len
                    .fetch_sub(prev.byte_len() as u64, Ordering::Relaxed);
            };
        }

        let mut over_byte_len =
            self.stored_byte_len.load(Ordering::Relaxed) as i64 - self.total_max_byte_len as i64;
        if over_byte_len > 0 {
            let mut sorted = inner
                .iter()
                .map(|(key, value)| (*key, value.last_accessed.load(Ordering::Relaxed)))
                .collect::<Vec<_>>();
            sorted.sort_by_key(|(_, last_accessed)| *last_accessed);

            let mut rev = sorted.into_iter().rev().map(|(key, _)| key);

            while over_byte_len > 0 {
                let key = rev.next().unwrap();
                let value = inner.remove(&key).unwrap();
                over_byte_len -= value.byte_len() as i64;
            }

            self.stored_byte_len
                .fetch_sub((-over_byte_len) as u64, Ordering::Relaxed);
        }

        self.inner.store(inner.into());
    }

    fn update_last_accessed(&self, value: &Value) {
        value
            .last_accessed
            .store(self.elapsed_u64(), Ordering::Relaxed);
    }

    fn elapsed_u64(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}

#[derive(Debug)]
struct Value {
    bytes: Option<Bytes>,
    last_accessed: AtomicU64,
}
impl Value {
    fn byte_len(&self) -> usize {
        self.bytes.as_ref().map_or(0, |value| value.len())
    }
}
impl Clone for Value {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
            last_accessed: AtomicU64::new(
                self.last_accessed
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
        }
    }
}
