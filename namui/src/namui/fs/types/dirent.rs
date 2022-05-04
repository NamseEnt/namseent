use std::path::PathBuf;

#[derive(Clone, Copy)]
pub enum DirentKind {
    Directory,
    File,
}

pub struct Dirent {
    path: PathBuf,
    kind: DirentKind,
}

impl Dirent {
    pub fn new(path: PathBuf, kind: DirentKind) -> Self {
        Self { path, kind }
    }

    pub fn is_dir(&self) -> bool {
        match self.kind {
            DirentKind::Directory => true,
            _ => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self.kind {
            DirentKind::File => true,
            _ => false,
        }
    }

    pub fn kind(&self) -> &DirentKind {
        &self.kind
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
