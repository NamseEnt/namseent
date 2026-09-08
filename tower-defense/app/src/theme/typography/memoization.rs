use crate::theme::typography::{PositionedRichText, TypographyBuilder};
use namui::*;

pub struct MemoizedBuilder<Deps, F> {
    deps: Deps,
    builder: F,
}

impl<Deps, F> MemoizedBuilder<Deps, F>
where
    Deps: TrackEqTuple,
    F: Fn(TypographyBuilder) -> PositionedRichText,
{
    pub fn new(deps: Deps, builder: F) -> Self {
        Self { deps, builder }
    }
}

impl<Deps, F> Component for MemoizedBuilder<Deps, F>
where
    Deps: TrackEqTuple,
    F: Fn(TypographyBuilder) -> PositionedRichText,
{
    fn render(self, ctx: &RenderCtx) {
        // arena-no-memo variant: the rendering tree lives in the per-frame
        // arena, so it cannot be cached across frames. Rebuild every frame.
        let _ = &self.deps;
        ctx.add((self.builder)(TypographyBuilder::new()));
    }
}

pub fn memoized_text<Deps, F>(deps: Deps, builder: F) -> MemoizedBuilder<Deps, F>
where
    Deps: TrackEqTuple,
    F: Fn(TypographyBuilder) -> PositionedRichText,
{
    MemoizedBuilder::new(deps, builder)
}
