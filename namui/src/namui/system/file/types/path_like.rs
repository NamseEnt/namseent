use super::Dirent;
use percent_encoding::percent_decode_str;
use std::path::PathBuf;
use url::Url;

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
        self.path_buf()
    }
}
impl PathLike for &str {
    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }
}
impl PathLike for Url {
    fn path(&self) -> PathBuf {
        assert!(self.cannot_be_a_base());
        let path = self.path();
        PathBuf::from(
            percent_decode_str(path)
                .decode_utf8()
                .expect(&format!("invalid url path: {}", path))
                .into_owned(),
        )
    }
}
