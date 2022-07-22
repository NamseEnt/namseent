use crate::{Matrix3x3, OneZero, Shader};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum ImageRenderable {
    Matrix { matrix: Matrix3x3, opacity: OneZero },
    Shader { shader: Arc<Shader> },
}
