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
        let (rendered, set_rendered) = ctx.state(|| None);
        let deps_changed = ctx.track_eq_tuple(&self.deps);

        if deps_changed || rendered.is_none() {
            set_rendered.set(Some((self.builder)(TypographyBuilder::new())));
        }

        ctx.add(rendered.clone_inner());
    }
}

pub fn memoized_text<Deps, F>(deps: Deps, builder: F) -> MemoizedBuilder<Deps, F>
where
    Deps: TrackEqTuple,
    F: Fn(TypographyBuilder) -> PositionedRichText,
{
    MemoizedBuilder::new(deps, builder)
}
