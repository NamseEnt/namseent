#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum ChildKey {
    Usize(usize),
    String(smol_str::SmolStr),
    U128(u128),
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

impl From<u128> for ChildKey {
    fn from(value: u128) -> Self {
        ChildKey::U128(value)
    }
}

impl From<&u128> for ChildKey {
    fn from(value: &u128) -> Self {
        ChildKey::U128(*value)
    }
}

pub enum AddKey {
    Usize(usize),
    String(smol_str::SmolStr),
    U128(u128),
    Incremental,
}

impl From<Option<AddKey>> for AddKey {
    fn from(key: Option<AddKey>) -> Self {
        key.unwrap_or(AddKey::Incremental)
    }
}

impl From<String> for AddKey {
    fn from(key: String) -> Self {
        AddKey::String(key.into())
    }
}

impl From<&str> for AddKey {
    fn from(key: &str) -> Self {
        AddKey::String(key.into())
    }
}

impl From<usize> for AddKey {
    fn from(key: usize) -> Self {
        AddKey::Usize(key)
    }
}
