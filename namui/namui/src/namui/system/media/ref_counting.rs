pub(crate) struct RefCounting<T> {
    value: T,
    ref_count: usize,
}

impl<T> RefCounting<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value,
            ref_count: 1,
        }
    }
    pub(crate) fn increase_ref_count(&mut self) {
        self.ref_count += 1;
    }
    pub(crate) fn decrease_ref_count(&mut self) {
        self.ref_count -= 1;
    }
    pub(crate) fn is_ref_count_zero(&self) -> bool {
        self.ref_count == 0
    }
}

impl<T: Clone> RefCounting<T> {
    pub(crate) fn inner_clone(&self) -> T {
        self.value.clone()
    }
}

impl<T> std::ops::Deref for RefCounting<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
