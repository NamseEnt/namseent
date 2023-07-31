use namui::prelude::*;
use rpc::data::CgFile;

pub static CG_FILES_ATOM: Atom<Vec<CgFile>> = Atom::uninitialized_new();
