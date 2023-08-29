use super::*;
use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct TextInputInstance {
    pub(crate) id: Uuid,
}

impl TextInputInstance {
    pub fn new(ctx: &RenderCtx) -> Self {
        TEXT_INPUT_ATOM.get_or_init(Default::default);
        let id = ctx.memo(|| uuid());

        TextInputInstance { id: *id }
    }

    pub fn focus(&self) {
        let id = self.id;
        TEXT_INPUT_ATOM.mutate(move |text_input| {
            *text_input = TextInputCtx {
                focus_request: Some(FocusRequest {
                    id,
                    focus_by: FocusBy::Api,
                }),
                ..Default::default()
            };
        });
    }

    pub fn blur(&self) {
        let id = self.id;
        TEXT_INPUT_ATOM.mutate(move |text_input| {
            if text_input.focused_id() == Some(id) {
                *text_input = Default::default();
            }
        });
    }

    pub fn focused(&self) -> bool {
        TEXT_INPUT_ATOM.get().is_focused(self.id)
    }
}
