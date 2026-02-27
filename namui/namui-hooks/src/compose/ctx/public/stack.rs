use super::*;
use crate::*;

impl ComposeCtx<'_, '_> {
    fn push(&self, command: ComposeCommand) -> Self {
        Self {
            composer: self.composer,
            world: self.world,
            // NOTE: Optimize point: `stack` can be replace with the index of full_stack.
            rt_container: self.rt_container,
            stack_parent_len: self.stack_parent_len,
            full_stack: {
                let mut full_stack = self.full_stack.clone().into_owned();
                full_stack.push(command);
                Cow::Owned(full_stack)
            },
        }
    }
    pub fn translate(&self, xy: impl Into<Xy<Px>>) -> Self {
        let xy = xy.into();
        self.push(ComposeCommand::Translate { xy })
    }
    pub fn absolute(&self, xy: impl Into<Xy<Px>>) -> Self {
        let xy = xy.into();
        self.push(ComposeCommand::Absolute { xy })
    }
    pub fn clip(&self, path: Path, clip_op: ClipOp) -> Self {
        self.push(ComposeCommand::Clip { path, clip_op })
    }
    pub fn on_top(&self) -> Self {
        self.push(ComposeCommand::OnTop)
    }
    pub fn rotate(&self, angle: Angle) -> Self {
        self.push(ComposeCommand::Rotate { angle })
    }
    pub fn scale(&self, scale_xy: Xy<f32>) -> Self {
        self.push(ComposeCommand::Scale { scale_xy })
    }
    pub fn mouse_cursor(&self, cursor: MouseCursor) -> Self {
        self.push(ComposeCommand::MouseCursor { cursor })
    }

    pub fn accumulated_matrix(&self) -> TransformMatrix {
        let mut matrix = TransformMatrix::identity();
        for command in self.full_stack.iter() {
            match command {
                ComposeCommand::Translate { xy } => {
                    matrix = matrix * TransformMatrix::from_translate(xy.x.as_f32(), xy.y.as_f32());
                }
                ComposeCommand::Absolute { xy } => {
                    matrix = TransformMatrix::from_translate(xy.x.as_f32(), xy.y.as_f32());
                }
                ComposeCommand::Rotate { angle } => {
                    matrix = matrix * TransformMatrix::from_rotate(*angle);
                }
                ComposeCommand::Scale { scale_xy } => {
                    matrix = matrix * TransformMatrix::from_scale(scale_xy.x, scale_xy.y);
                }
                ComposeCommand::Clip { .. }
                | ComposeCommand::OnTop
                | ComposeCommand::MouseCursor { .. } => {}
            }
        }
        matrix
    }
}
