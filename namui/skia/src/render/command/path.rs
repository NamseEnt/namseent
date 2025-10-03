use crate::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq, bincode::Encode, bincode::Decode)]
pub struct PathDrawCommand {
    pub path: Path,
    pub paint: Paint,
}
