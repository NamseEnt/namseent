#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct ComposerId {
    id: usize,
}

impl ComposerId {
    pub(crate) fn root() -> ComposerId {
        ComposerId { id: 0 }
    }
    pub(crate) fn generate() -> ComposerId {
        static ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

        ComposerId {
            id: ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }
}
