use crate::app::{editor::Editor, sequence_list::SequenceList};

pub enum Page {
    Editor(Editor),
    SequenceList(SequenceList),
}
