use crate::app::types::*;
use std::sync::Arc;

pub enum Clipboard {
    CameraClip(Arc<CameraClip>),
}
