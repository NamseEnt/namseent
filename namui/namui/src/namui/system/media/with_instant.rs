use namui_type::Instant;
use std::fmt::{Debug, Formatter};

pub(crate) struct WithInstant<T> {
    value: T,
    pub(crate) instant: crate::Instant,
}

impl<T> WithInstant<T> {
    pub(crate) fn new(value: T, instant: Instant) -> Self {
        Self { value, instant }
    }
}

impl<T> std::ops::Deref for WithInstant<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for WithInstant<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Debug> Debug for WithInstant<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithInstant")
            .field("value", &self.value)
            .field("instant", &self.instant)
            .finish()
    }
}

pub(crate) trait WithNow {
    fn with_now(self) -> WithInstant<Self>
    where
        Self: Sized,
    {
        WithInstant::new(self, crate::time::now())
    }
}

impl<T> WithNow for T {}

pub(crate) trait WithInstantExt {
    fn with_instant(self, instant: Instant) -> WithInstant<Self>
    where
        Self: Sized,
    {
        WithInstant::new(self, instant)
    }
}

impl<T> WithInstantExt for T {}
