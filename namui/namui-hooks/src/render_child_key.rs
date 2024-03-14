use namui_type::Uuid;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ChildKey {
    Usize(usize),
    String(smol_str::SmolStr),
    Uuid(Uuid),
    IncrementalComponent {
        index: usize,
        type_name: &'static str,
    },
    IncrementalCompose {
        index: usize,
    },
}

impl From<String> for ChildKey {
    fn from(value: String) -> Self {
        ChildKey::String(smol_str::SmolStr::new(value))
    }
}

impl<'a> From<&'a String> for ChildKey {
    fn from(value: &'a String) -> Self {
        ChildKey::String(smol_str::SmolStr::new(value))
    }
}

impl<'a> From<&'a str> for ChildKey {
    fn from(value: &'a str) -> Self {
        ChildKey::String(smol_str::SmolStr::new(value))
    }
}

impl From<usize> for ChildKey {
    fn from(value: usize) -> Self {
        ChildKey::Usize(value)
    }
}

impl From<Uuid> for ChildKey {
    fn from(value: Uuid) -> Self {
        ChildKey::Uuid(value)
    }
}

impl From<&Uuid> for ChildKey {
    fn from(value: &Uuid) -> Self {
        ChildKey::Uuid(*value)
    }
}
