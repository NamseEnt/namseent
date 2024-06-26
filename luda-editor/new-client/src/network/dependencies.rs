use namui::*;

pub trait Dependencies {
    fn changed(&self, ctx: &RenderCtx) -> bool;
}

impl<T: 'static + PartialEq + Clone> Dependencies for Option<&T> {
    fn changed(&self, ctx: &RenderCtx) -> bool {
        let (state, set_state) = ctx.state(|| self.cloned());
        if *self != state.as_ref().as_ref() {
            set_state.set(self.cloned());
            true
        } else {
            false
        }
    }
}

impl Dependencies for () {
    fn changed(&self, _: &RenderCtx) -> bool {
        false
    }
}

impl<T0, T1> Dependencies for (T0, T1)
where
    T0: Dependencies,
    T1: Dependencies,
{
    fn changed(&self, ctx: &RenderCtx) -> bool {
        self.0.changed(ctx) || self.1.changed(ctx)
    }
}

impl<T0, T1, T2> Dependencies for (T0, T1, T2)
where
    T0: Dependencies,
    T1: Dependencies,
    T2: Dependencies,
{
    fn changed(&self, ctx: &RenderCtx) -> bool {
        self.0.changed(ctx) || self.1.changed(ctx) || self.2.changed(ctx)
    }
}

impl<T0, T1, T2, T3> Dependencies for (T0, T1, T2, T3)
where
    T0: Dependencies,
    T1: Dependencies,
    T2: Dependencies,
    T3: Dependencies,
{
    fn changed(&self, ctx: &RenderCtx) -> bool {
        self.0.changed(ctx) || self.1.changed(ctx) || self.2.changed(ctx) || self.3.changed(ctx)
    }
}

impl<T0, T1, T2, T3, T4> Dependencies for (T0, T1, T2, T3, T4)
where
    T0: Dependencies,
    T1: Dependencies,
    T2: Dependencies,
    T3: Dependencies,
    T4: Dependencies,
{
    fn changed(&self, ctx: &RenderCtx) -> bool {
        self.0.changed(ctx)
            || self.1.changed(ctx)
            || self.2.changed(ctx)
            || self.3.changed(ctx)
            || self.4.changed(ctx)
    }
}

impl<T0, T1, T2, T3, T4, T5> Dependencies for (T0, T1, T2, T3, T4, T5)
where
    T0: Dependencies,
    T1: Dependencies,
    T2: Dependencies,
    T3: Dependencies,
    T4: Dependencies,
    T5: Dependencies,
{
    fn changed(&self, ctx: &RenderCtx) -> bool {
        self.0.changed(ctx)
            || self.1.changed(ctx)
            || self.2.changed(ctx)
            || self.3.changed(ctx)
            || self.4.changed(ctx)
            || self.5.changed(ctx)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> Dependencies for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Dependencies,
    T1: Dependencies,
    T2: Dependencies,
    T3: Dependencies,
    T4: Dependencies,
    T5: Dependencies,
    T6: Dependencies,
{
    fn changed(&self, ctx: &RenderCtx) -> bool {
        self.0.changed(ctx)
            || self.1.changed(ctx)
            || self.2.changed(ctx)
            || self.3.changed(ctx)
            || self.4.changed(ctx)
            || self.5.changed(ctx)
            || self.6.changed(ctx)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Dependencies for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Dependencies,
    T1: Dependencies,
    T2: Dependencies,
    T3: Dependencies,
    T4: Dependencies,
    T5: Dependencies,
    T6: Dependencies,
    T7: Dependencies,
{
    fn changed(&self, ctx: &RenderCtx) -> bool {
        self.0.changed(ctx)
            || self.1.changed(ctx)
            || self.2.changed(ctx)
            || self.3.changed(ctx)
            || self.4.changed(ctx)
            || self.5.changed(ctx)
            || self.6.changed(ctx)
            || self.7.changed(ctx)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Dependencies for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: Dependencies,
    T1: Dependencies,
    T2: Dependencies,
    T3: Dependencies,
    T4: Dependencies,
    T5: Dependencies,
    T6: Dependencies,
    T7: Dependencies,
    T8: Dependencies,
{
    fn changed(&self, ctx: &RenderCtx) -> bool {
        self.0.changed(ctx)
            || self.1.changed(ctx)
            || self.2.changed(ctx)
            || self.3.changed(ctx)
            || self.4.changed(ctx)
            || self.5.changed(ctx)
            || self.6.changed(ctx)
            || self.7.changed(ctx)
            || self.8.changed(ctx)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Dependencies
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: Dependencies,
    T1: Dependencies,
    T2: Dependencies,
    T3: Dependencies,
    T4: Dependencies,
    T5: Dependencies,
    T6: Dependencies,
    T7: Dependencies,
    T8: Dependencies,
    T9: Dependencies,
{
    fn changed(&self, ctx: &RenderCtx) -> bool {
        self.0.changed(ctx)
            || self.1.changed(ctx)
            || self.2.changed(ctx)
            || self.3.changed(ctx)
            || self.4.changed(ctx)
            || self.5.changed(ctx)
            || self.6.changed(ctx)
            || self.7.changed(ctx)
            || self.8.changed(ctx)
            || self.9.changed(ctx)
    }
}
