use crate::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct PathDrawCommand {
    pub path: Path,
    pub paint: Paint,
}
