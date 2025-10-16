use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct ComposerId {
    id: usize,
}

impl ComposerId {
    pub(crate) const ROOT: ComposerId = ComposerId { id: 0 };
    pub(crate) fn generate() -> ComposerId {
        static ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

        ComposerId {
            id: ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, OurSerde)]
#[repr(transparent)]
pub(crate) struct InstanceId {
    id: usize,
}

impl InstanceId {
    pub(crate) const ROOT: InstanceId = InstanceId { id: 0 };
    pub(crate) fn generate() -> InstanceId {
        static ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

        InstanceId {
            id: ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }
}
