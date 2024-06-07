use crate::*;

#[type_derives(Hash, Eq, -serde::Serialize, -serde::Deserialize)]
pub struct PathDrawCommand {
    pub path: Path,
    pub paint: Paint,
}
