use std::path::PathBuf;

pub trait PathLike {
    fn path(&self) -> PathBuf;
}
impl PathLike for &PathBuf {
    fn path(&self) -> PathBuf {
        self.to_path_buf()
    }
}
impl PathLike for PathBuf {
    fn path(&self) -> PathBuf {
        self.clone()
    }
}
impl PathLike for &str {
    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }
}
impl PathLike for &String {
    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }
}
impl PathLike for String {
    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }
}
