use namui::prelude::*;
use rpc::data::CgFile;

pub static CG_FILES_ATOM: Atom<Vec<CgFile>> = Atom::uninitialized_new();

pub(crate) trait UpdateCgFile {
    fn update_file(&mut self, cg_file: CgFile);
}

impl UpdateCgFile for Vec<CgFile> {
    fn update_file(&mut self, cg_file: CgFile) {
        if let Some(index) = self.iter().position(|cg_file_| cg_file_.id == cg_file.id) {
            self[index] = cg_file;
        } else {
            self.push(cg_file);
        }
    }
}
