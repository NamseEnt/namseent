use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct TextInputInstance {
    pub(crate) id: Uuid,
}

impl TextInputInstance {
    pub fn new(ctx: &RenderCtx) -> Self {
        let id = ctx.memo(|| uuid());

        TextInputInstance { id: *id }
    }

    pub fn focus(&self) {
        todo!()
    }

    pub fn blur(&self) {
        todo!()
    }
}
