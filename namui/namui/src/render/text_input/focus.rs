use std::sync::atomic::AtomicBool;

pub struct TextInputFocus {
    pub(crate) on: AtomicBool,
}

impl TextInputFocus {
    pub fn new() -> Self {
        Self {
            on: AtomicBool::new(false),
        }
    }

    pub fn on(&self) {
        self.on.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn off(&self) {
        self.on.store(false, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn focused(&self) -> bool {
        self.on.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for TextInputFocus {
    fn default() -> Self {
        Self::new()
    }
}
