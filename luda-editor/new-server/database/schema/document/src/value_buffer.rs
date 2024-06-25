use std::sync::Arc;

pub enum ValueBuffer {
    Vec(Vec<u8>),
    Arc(Arc<Vec<u8>>),
}
impl ValueBuffer {
    pub fn get_arc_vec(&self) -> Arc<Vec<u8>> {
        match self {
            Self::Vec(vec) => Arc::new(vec.clone()),
            Self::Arc(arc) => arc.clone(),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::Vec(vec) => vec.as_slice(),
            Self::Arc(arc) => arc.as_slice(),
        }
    }
}
impl From<Vec<u8>> for ValueBuffer {
    fn from(vec: Vec<u8>) -> Self {
        Self::Vec(vec)
    }
}
impl From<Arc<Vec<u8>>> for ValueBuffer {
    fn from(arc: Arc<Vec<u8>>) -> Self {
        Self::Arc(arc)
    }
}
