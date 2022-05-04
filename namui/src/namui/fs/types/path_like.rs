use super::Dirent;
use std::path::PathBuf;

pub trait PathLike {
    fn path(&self) -> PathBuf;
}
impl PathLike for PathBuf {
    fn path(&self) -> PathBuf {
        self.clone()
    }
}
impl PathLike for Dirent {
    fn path(&self) -> PathBuf {
        self.path().clone()
    }
}
impl PathLike for &str {
    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }
}
