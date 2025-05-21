use crate::*;

#[derive(Debug, Clone)]
pub enum ComposeCommand {
    Translate { xy: Xy<Px> },
    Absolute { xy: Xy<Px> },
    Clip { path: Path, clip_op: ClipOp },
    OnTop,
    Rotate { angle: Angle },
    Scale { scale_xy: Xy<f32> },
    MouseCursor { cursor: MouseCursor },
}
