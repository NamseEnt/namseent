use percent_encoding::percent_decode_str;
use std::{borrow::Cow, hash::Hash, path::PathBuf};
use url::Url;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DirentKind {
    Directory,
    File,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Dirent {
    Directory(Url),
    File(Url),
}

impl Dirent {
    #[allow(dead_code)]
    pub(crate) fn new(url: Url, kind: DirentKind) -> Dirent {
        match kind {
            DirentKind::Directory => Dirent::Directory(url),
            DirentKind::File => Dirent::File(url),
        }
    }
    pub fn is_dir(&self) -> bool {
        matches!(self, Dirent::Directory(_))
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Dirent::File(_))
    }

    pub fn kind(&self) -> DirentKind {
        match self {
            Dirent::Directory(_) => DirentKind::Directory,
            Dirent::File(_) => DirentKind::File,
        }
    }

    pub fn url(&self) -> &Url {
        match self {
            Dirent::Directory(url) => url,
            Dirent::File(url) => url,
        }
    }

    pub fn path_string(&self) -> Cow<'_, str> {
        percent_decode_str(self.url().path())
            .decode_utf8()
            .expect("invalid url path")
    }

    pub fn path_buf(&self) -> PathBuf {
        PathBuf::from(self.path_string().to_string())
    }
}
