pub enum EffectCleanUp {
    None,
    Once(Box<dyn FnOnce()>),
}

impl EffectCleanUp {
    pub fn once(f: impl FnOnce() + 'static) -> Self {
        Self::Once(Box::new(f))
    }
    pub(crate) fn call(self) {
        match self {
            Self::None => {}
            Self::Once(f) => f(),
        }
    }

    pub(crate) fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}

impl Default for EffectCleanUp {
    fn default() -> Self {
        Self::None
    }
}

impl From<()> for EffectCleanUp {
    fn from(_value: ()) -> Self {
        Self::None
    }
}

impl<T: FnOnce() + 'static> From<T> for EffectCleanUp {
    fn from(value: T) -> Self {
        Self::Once(Box::new(value))
    }
}
