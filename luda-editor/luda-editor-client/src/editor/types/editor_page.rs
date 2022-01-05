use super::Sequence;

pub enum EditorPage {
    Editor,
    SequenceListView,
}

pub enum EditorPageChangeEventDetail {
    Editor {
        path: String,
        sequence: Sequence,
    },
    // TODO: It will be used to make back button in editor page
    #[allow(dead_code)]
    SequenceListView,
}
