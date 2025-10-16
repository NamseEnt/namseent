use crate::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq, State)]
pub struct PathDrawCommand {
    pub path: Path,
    pub paint: Paint,
}
