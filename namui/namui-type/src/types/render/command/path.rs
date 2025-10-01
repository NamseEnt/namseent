use crate::*;

#[derive(Debug, bincode::Decode, bincode::Encode, PartialEq, Clone, Hash, Eq)]
pub struct PathDrawCommand {
    pub path: Path,
    pub paint: Paint,
}
