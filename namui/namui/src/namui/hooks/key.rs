use namui_type::Uuid;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Key {
    Usize(usize),
    String(smol_str::SmolStr),
    Uuid(Uuid),
    IncrementalComponent { index: usize, name: &'static str },
    IncrementalCompose { index: usize },
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Key::String(smol_str::SmolStr::new(value))
    }
}

impl<'a> From<&'a String> for Key {
    fn from(value: &'a String) -> Self {
        Key::String(smol_str::SmolStr::new(value))
    }
}

impl<'a> From<&'a str> for Key {
    fn from(value: &'a str) -> Self {
        Key::String(smol_str::SmolStr::new(value))
    }
}

impl From<usize> for Key {
    fn from(value: usize) -> Self {
        Key::Usize(value)
    }
}

impl From<Uuid> for Key {
    fn from(value: Uuid) -> Self {
        Key::Uuid(value)
    }
}

impl From<&Uuid> for Key {
    fn from(value: &Uuid) -> Self {
        Key::Uuid(*value)
    }
}
