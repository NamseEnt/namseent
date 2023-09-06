use namui_type::Uuid;

#[document_macro::document]
/// SequenceIndexDocument is a document that contains the most recent index of the sequence.
pub struct SequenceIndexDocument {
    #[pk]
    pub id: rpc::Uuid,

    pub project_id: rpc::Uuid,
    /// The index of the most recent sequence version.
    pub index: CircularIndex<8>,
    pub undoable_count: BoundedUsize<8>,
    pub redoable_count: BoundedUsize<8>,
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
    pub fn increase(&mut self) {
        self.index = (self.index + 1) % N;
    }
    pub fn decrease(&mut self) {
        self.index = (self.index + N - 1) % N;
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct BoundedUsize<const N: usize> {
    value: usize,
}

impl<const N: usize> BoundedUsize<N> {
    pub const fn new() -> Self {
        Self { value: 0 }
    }
    pub fn make_zero(&mut self) {
        self.value = 0;
    }
    pub fn increase(&mut self) {
        if self.value < N - 1 {
            self.value += 1;
        }
    }
    pub fn decrease(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }
}

impl<const N: usize> std::ops::Deref for BoundedUsize<N> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
