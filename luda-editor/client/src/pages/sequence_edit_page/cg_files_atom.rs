use crate::atom::{Atomic, OptionAtom};
use rpc::data::CgFile;
use std::ops::Deref;

impl Atomic for CgFilesAtom {
    fn on_update(&self) {}
}

pub struct CgFilesAtom {
    cg_files: Vec<CgFile>,
}

impl CgFilesAtom {
    pub fn new(cg_files: Vec<CgFile>) -> Self {
        Self { cg_files }
    }

    pub(crate) fn update_file(&mut self, cg_file: CgFile) {
        if let Some(index) = self.iter().position(|cg_file_| cg_file_.id == cg_file.id) {
            self.cg_files[index] = cg_file;
        } else {
            self.cg_files.push(cg_file);
        }
    }
}

impl Deref for CgFilesAtom {
    type Target = Vec<CgFile>;

    fn deref(&self) -> &Self::Target {
        &self.cg_files
    }
}

pub static CG_FILES_ATOM: OptionAtom<CgFilesAtom> = OptionAtom::new();
