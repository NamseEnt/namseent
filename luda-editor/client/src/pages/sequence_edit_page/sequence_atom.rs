use super::sequence::SequenceWrapped;
use crate::atom::{Atomic, OptionAtom};

impl Atomic for SequenceWrapped {
    fn on_update(&self) {}
}

pub static SEQUENCE_ATOM: OptionAtom<SequenceWrapped> = OptionAtom::new();
