use crate::*;

#[type_derives(-serde::Deserialize)]
pub struct PathDrawCommand {
    pub path: Path,
    pub paint: Paint,
}
