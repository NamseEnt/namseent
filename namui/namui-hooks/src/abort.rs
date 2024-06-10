pub trait Abort {
    fn abort(&self);
}

impl<T> Abort for tokio::task::JoinHandle<T> {
    fn abort(&self) {
        self.abort();
    }
}

impl Abort for tokio::task::AbortHandle {
    fn abort(&self) {
        self.abort();
    }
}
