use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dirent {
    pub name: String,
    pub file_type: DirentFileType,
}

#[derive(Serialize, Deserialize)]
pub enum DirentFileType {
    File,
    Directory,
}
