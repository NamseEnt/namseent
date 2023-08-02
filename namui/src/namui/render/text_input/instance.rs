use crate::*;

#[derive(Clone, Debug)]
pub struct TextInputInstance {
    pub(crate) id: Uuid,
}

impl TextInputInstance {
    pub fn new(ctx: &RenderCtx) -> Self {
        let id = ctx.use_memo(|| uuid());

        TextInputInstance { id: *id }
    }

    pub fn focus(&self) {
        todo!()
    }

    pub fn blur(&self) {
        todo!()
    }
}
