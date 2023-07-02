use namui_type::Uuid;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct CircularIndex<const N: usize> {
    index: usize,
}

impl<const N: usize> std::fmt::Display for CircularIndex<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

impl<const N: usize> CircularIndex<N> {
    pub const fn new() -> Self {
        Self { index: 0 }
    }
    pub fn next(&self) -> Self {
        Self {
            index: (self.index + 1) % N,
        }
    }
    pub fn increase(&mut self) {
        *self = self.next();
    }
    pub fn decrease(&self) -> Self {
        Self {
            index: (self.index + N - 1) % N,
        }
    }
}

#[document_macro::document]
pub struct SequenceIndexDocument {
    #[pk]
    pub id: rpc::Uuid,

    pub project_id: rpc::Uuid,
    pub index: CircularIndex<8>,
    pub undoable_count: usize,
    pub redoable_count: usize,
}

#[document_macro::document]
pub struct SequenceDocument {
    #[pk]
    pub id: rpc::Uuid,

    #[sk]
    pub index: CircularIndex<8>,

    pub project_id: rpc::Uuid,
    pub name: String,
    pub cuts: Vec<CutIndex>,
}

impl SequenceDocument {
    pub fn cut_mut(&mut self, cut_id: Uuid) -> Option<&mut CutIndex> {
        self.cuts.iter_mut().find(|c| c.cut_id == cut_id)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CutIndex {
    pub cut_id: Uuid,
    pub index: CircularIndex<8>,
}

impl SequenceDocument {
    pub fn update(&mut self) {}
}

#[document_macro::document]
pub struct SequenceCutDocument {
    #[pk]
    pub sequence_id: rpc::Uuid,
    #[pk]
    pub cut_id: rpc::Uuid,
    #[sk]
    pub cut_index: CircularIndex<8>,

    pub cut: rpc::data::Cut,
}

#[document_macro::document]
pub struct ProjectSequenceDocument {
    #[pk]
    pub project_id: rpc::Uuid,
    #[sk]
    pub sequence_id: rpc::Uuid,
}
