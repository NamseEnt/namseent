use crate::app::types::*;
use std::sync::Arc;

pub enum Clipboard {
    BackgroundClip(Arc<BackgroundClip>),
    CameraClip(Arc<CameraClip>),
}
