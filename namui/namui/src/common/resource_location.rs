use crate::*;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
// use url::Url;

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, State)]
pub enum ResourceLocation {
    Bundle(PathBuf),
    KvStore(String),
    // Network(Url),
}

impl ResourceLocation {
    pub fn bundle(path: impl AsRef<Path>) -> Self {
        Self::Bundle(path.as_ref().to_path_buf())
    }
}

impl AsRef<ResourceLocation> for ResourceLocation {
    fn as_ref(&self) -> &ResourceLocation {
        self
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceLocation::Bundle(path) => write!(f, "Bundle>> {}", path.display()),
            ResourceLocation::KvStore(key) => write!(f, "KvStore>> {key}"),
            // ResourceLocation::Network(url) => write!(f, "Network>> {url}"),
        }
    }
}
