use crate::app::{authentication::Authentication, editor::Editor, sequence_list::SequenceList};

pub enum Page {
    Editor(Editor),
    SequenceList(SequenceList),
    Authentication(Authentication),
}
